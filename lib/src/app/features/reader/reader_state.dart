import 'package:flutter/widgets.dart';
import 'package:textrest/src/rust/api/book.dart';
import 'package:textrest/src/rust/api/read_record.dart';
import 'package:textrest/src/rust/api/web_book.dart';
import 'package:textrest/src/rust/models/book.dart';
import 'package:textrest/src/rust/models/book_source.dart';
import 'package:textrest/src/rust/models/chapter.dart';
import 'package:textrest/src/rust/models/read_record.dart';

class ReaderState with ChangeNotifier {
  final String bookId;
  static const int _continuousLoadWindow = 4;

  ReaderState({required this.bookId});

  BookApi? _bookApi;
  ReadRecordApi? _readRecordApi;
  WebBookApi? _webBookApi;

  Book? _book;
  List<Chapter> _chapters = [];
  ReadRecord? _readRecord;
  int _currentChapterIndex = 0;
  double _scrollPosition = 0;
  int _currentChapterCharPosition = 0;
  bool _isLoading = true;
  String? _error;
  String? _chapterContent;
  DateTime? _startReadingTime;
  DateTime? _lastSavedAt;
  int _lastSavedChapterIndex = 0;
  double _lastSavedPosition = 0;
  bool _isInitialized = false;
  BookSource? _bookSource;
  final Map<int, String?> _chapterContentCache = {};
  final Map<int, double> _chapterScrollPositions = {};
  final Map<int, int> _chapterCharPositions = {};

  Book? get book => _book;
  List<Chapter> get chapters => _chapters;
  ReadRecord? get readRecord => _readRecord;
  int get currentChapterIndex => _currentChapterIndex;
  double get scrollPosition => _scrollPosition;
  int get currentChapterCharPosition => _currentChapterCharPosition;
  bool get isLoading => _isLoading;
  String? get error => _error;
  String? get chapterContent => _chapterContent;
  bool get hasPrevChapter => _currentChapterIndex > 0;
  bool get hasNextChapter => _currentChapterIndex < _chapters.length - 1;
  Chapter? get currentChapter =>
      _chapters.isNotEmpty && _currentChapterIndex < _chapters.length
          ? _chapters[_currentChapterIndex]
          : null;

  double getChapterScrollPosition(int index) => _chapterScrollPositions[index] ?? 0;
  int getChapterCharPosition(int index) => _chapterCharPositions[index] ?? 0;

  bool get isNetworkBook => _book?.bookType != 'epub' && _book?.bookType != 'pdf' && _book?.bookType != 'txt' && _book?.bookType != 'mobi';

  Future<void> initialize() async {
    if (_isInitialized) return;

    _startReadingTime = DateTime.now();
    _isLoading = true;
    _error = null;
    notifyListeners();

    try {
      _bookApi ??= await BookApi.newInstance();
      _readRecordApi ??= await ReadRecordApi.newInstance();
      _webBookApi ??= await WebBookApi.newInstance();

      _book = await _bookApi!.getBook(bookId: bookId);
      if (_book == null) {
        throw Exception('书籍不存在');
      }

      _chapters = await _bookApi!.loadBookToc(bookId: bookId);
      _readRecord = await _readRecordApi!.getRecordByBookId(bookId: bookId);

      if (_readRecord != null) {
        _currentChapterIndex = _readRecord!.durChapterIndex.toInt();
        _currentChapterCharPosition = _readRecord!.durChapterPos.toInt();
        _chapterCharPositions[_currentChapterIndex] = _currentChapterCharPosition;
        _lastSavedChapterIndex = _currentChapterIndex;
        _lastSavedPosition = _currentChapterCharPosition.toDouble();
        _lastSavedAt = DateTime.now();
      }

      if (isNetworkBook) {
        await _loadBookSource();
      }

      if (_chapters.isEmpty) {
        _error = '书籍没有章节';
      } else if (_currentChapterIndex >= _chapters.length) {
        _currentChapterIndex = 0;
        await _loadChapterContent(0);
      } else {
        await _loadChapterContent(_currentChapterIndex);
      }

      _isInitialized = true;
    } catch (e) {
      _error = '初始化失败: $e';
    } finally {
      _isLoading = false;
      notifyListeners();
    }
  }

  Future<void> _loadBookSource() async {
    if (_book == null || _webBookApi == null) return;

    try {
      final bookSources = await _webBookApi!.loadBookSources();
      if (bookSources.isEmpty) {
        throw Exception('没有可用的书源');
      }

      final origin = _book!.origin?.trim();
      BookSource? selectedSource;
      
      if (origin != null && origin.isNotEmpty) {
        selectedSource = bookSources.firstWhere(
          (s) => s.bookSourceUrl == origin || s.bookSourceName == origin,
          orElse: () => bookSources.first,
        );
      } else {
        selectedSource = bookSources.first;
      }

      _bookSource = selectedSource;
    } catch (e) {
      _error = '加载书源失败: $e';
    }
  }

  Future<void> _loadChapterContent(int index) async {
    if (_bookApi == null || _book == null) {
      _error = 'API未初始化';
      notifyListeners();
      return;
    }

    if (index < 0 || index >= _chapters.length) {
      _error = '章节索引超出范围: $index, 总章节: ${_chapters.length}';
      notifyListeners();
      return;
    }

    try {
      if (_chapterContentCache.containsKey(index)) {
        _chapterContent = _chapterContentCache[index];
      } else {
        if (isNetworkBook) {
          final chapter = _chapters[index];
          if (_bookSource != null && chapter.url.isNotEmpty) {
            final chapterContent = await _webBookApi!.getChapterContent(
              bookId: bookId,
              bookSource: _bookSource!,
              chapterUrl: chapter.url,
            );
            _chapterContent = chapterContent.content;
            _chapterContentCache[index] = chapterContent.content;
          } else {
            _chapterContent = '网络书籍缺少必要信息: bookSource=${_bookSource != null}, chapterUrl=${chapter.url}';
            _chapterContentCache[index] = _chapterContent;
          }
        } else {
          _chapterContent = await _bookApi!.loadChapterContent(
            bookId: bookId,
            chapterIndex: BigInt.from(index),
          );
          _chapterContentCache[index] = _chapterContent;
        }
      }
      _error = null;
      notifyListeners();
      _ensureInitialWindow(index);
    } catch (e) {
      _error = '加载章节内容失败: $e';
      _chapterContent = null;
      notifyListeners();
    }
  }

  Future<String?> getChapterContent(int index) async {
    if (_bookApi == null || _book == null) {
      return null;
    }
    
    if (index < 0 || index >= _chapters.length) {
      return null;
    }

    if (_chapterContentCache.containsKey(index)) {
      return _chapterContentCache[index];
    }

    try {
      String? content;
      if (isNetworkBook) {
        final chapter = _chapters[index];
        if (_bookSource != null && chapter.url.isNotEmpty) {
          final chapterContent = await _webBookApi!.getChapterContent(
            bookId: bookId,
            bookSource: _bookSource!,
            chapterUrl: chapter.url,
          );
          content = chapterContent.content;
        } else {
          content = '网络书籍缺少必要信息';
        }
      } else {
        content = await _bookApi!.loadChapterContent(
          bookId: bookId,
          chapterIndex: BigInt.from(index),
        );
      }
      _chapterContentCache[index] = content;
      return content;
    } catch (e) {
      return '加载章节内容失败: $e';
    }
  }

  String? getCachedChapterContent(int index) {
    return _chapterContentCache[index];
  }

  Future<void> _ensureInitialWindow(int anchorIndex) async {
    final minIndex = (anchorIndex - _continuousLoadWindow).clamp(0, _chapters.length - 1);
    final maxIndex = (anchorIndex + _continuousLoadWindow).clamp(0, _chapters.length - 1);

    for (int i = minIndex; i <= maxIndex; i++) {
      if (i == anchorIndex) continue;
      if (_chapterContentCache.containsKey(i)) continue;
      Future.microtask(() => _preloadChapter(i));
    }

    _trimCacheAround(anchorIndex);
  }

  Future<void> preloadAround(int anchorIndex) async {
    await _ensureInitialWindow(anchorIndex);
  }

  void _trimCacheAround(int anchorIndex) {
    final minIndex = (anchorIndex - _continuousLoadWindow).clamp(0, _chapters.length - 1);
    final maxIndex = (anchorIndex + _continuousLoadWindow).clamp(0, _chapters.length - 1);

    final keysToRemove = _chapterContentCache.keys
        .where((index) => index < minIndex || index > maxIndex)
        .toList();

    for (final index in keysToRemove) {
      _chapterContentCache.remove(index);
      _chapterScrollPositions.remove(index);
      _chapterCharPositions.remove(index);
    }
  }

  Future<void> _preloadChapter(int index) async {
    try {
      await getChapterContent(index);
    } catch (e) {
      // 预加载失败不影响用户体验，静默处理
    }
  }

  Future<void> goToChapter(int index) async {
    if (index < 0 || index >= _chapters.length) return;

    await _saveReadingProgress(pageChanged: true);
    _currentChapterIndex = index;
    _scrollPosition = _chapterScrollPositions[index] ?? 0;
    await _loadChapterContent(index);
  }

  Future<void> goToPrevChapter() async {
    if (!hasPrevChapter) return;
    await goToChapter(_currentChapterIndex - 1);
  }

  Future<void> goToNextChapter() async {
    if (!hasNextChapter) return;
    await goToChapter(_currentChapterIndex + 1);
  }

  void updateScrollPosition(double position) {
    updateChapterProgress(_currentChapterIndex, position);
  }

  void updateChapterProgress(int index, double position) {
    _scrollPosition = position;
    _chapterScrollPositions[index] = position;
    if (index == _currentChapterIndex) {
      _scrollPosition = position;
    }
  }

  void updateChapterCharProgress(int index, int charPosition) {
    _chapterCharPositions[index] = charPosition;
    if (index == _currentChapterIndex) {
      _currentChapterCharPosition = charPosition;
    }
  }

  bool shouldPersistProgress({bool pageChanged = false}) {
    final now = DateTime.now();
    final elapsed = _lastSavedAt == null ? 999999 : now.difference(_lastSavedAt!).inSeconds;
    final chapterChanged = _lastSavedChapterIndex != _currentChapterIndex;
    final positionDelta = (_currentChapterCharPosition.toDouble() - _lastSavedPosition).abs();

    return pageChanged || chapterChanged || elapsed >= 6 || positionDelta >= 120;
  }

  Future<void> _saveReadingProgress({bool pageChanged = false}) async {
    if (_readRecordApi == null || _book == null) return;
    if (!shouldPersistProgress(pageChanged: pageChanged)) return;

    final now = DateTime.now();
    int readTimeSeconds = 0;

    if (_startReadingTime != null) {
      readTimeSeconds = now.difference(_startReadingTime!).inSeconds;
    }

    final newReadRecord = ReadRecord(
      bookId: bookId,
      durChapterIndex: BigInt.from(_currentChapterIndex),
      durChapterPos: BigInt.from(_currentChapterCharPosition),
      lastChapterIndex: BigInt.from(_currentChapterIndex),
      lastChapterPos: BigInt.from(_currentChapterCharPosition),
      totalReadTime: BigInt.from((_readRecord?.totalReadTime.toInt() ?? 0) + readTimeSeconds),
      lastReadTime: _readRecord?.lastReadTime,
    );

    await _readRecordApi!.updateRecordByBookId(
      bookId: bookId,
      record: newReadRecord,
    );

    _readRecord = newReadRecord;
    _startReadingTime = now;
    _lastSavedAt = now;
    _lastSavedChapterIndex = _currentChapterIndex;
    _lastSavedPosition = _currentChapterCharPosition.toDouble();
  }

  Future<void> saveReadingProgress({bool pageChanged = false}) async {
    await _saveReadingProgress(pageChanged: pageChanged);
  }

  Future<void> saveAndDispose() async {
    await _saveReadingProgress();
  }
}
