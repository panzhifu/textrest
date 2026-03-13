import 'package:flutter/material.dart';
import 'package:flutter_widget_from_html/flutter_widget_from_html.dart';
import 'package:textrest/src/app/features/reader/chapter_list_dialog.dart';
import 'package:textrest/src/app/features/reader/reader_state.dart';
import 'package:textrest/src/app/features/reader/theme_settings_dialog.dart';
import 'package:textrest/src/rust/models/chapter.dart';

class ReaderPage extends StatefulWidget {
  final String bookId;

  const ReaderPage({
    super.key,
    required this.bookId,
  });

  @override
  State<ReaderPage> createState() => _ReaderPageState();
}

class _ReaderPageState extends State<ReaderPage> with WidgetsBindingObserver {
  late ReaderState _readerState;
  double _fontSize = 16;
  double _lineHeight = 1.6;
  ReadingTheme _theme = ReadingTheme.light;
  bool _showOptions = false;
  final ScrollController _scrollController = ScrollController();
  final Map<int, GlobalKey> _chapterKeys = {};
  final GlobalKey _listKey = GlobalKey();
  bool _isUpdatingCurrentChapter = false;
  DateTime? _lastTapTime;
  Offset? _lastTapPosition;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
    _readerState = ReaderState(bookId: widget.bookId);
    _readerState.initialize();
  }

  @override
  void dispose() {
    _scrollController.dispose();
    WidgetsBinding.instance.removeObserver(this);
    _readerState.saveAndDispose();
    super.dispose();
  }

  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    if (state == AppLifecycleState.paused ||
        state == AppLifecycleState.inactive ||
        state == AppLifecycleState.detached) {
      _readerState.saveAndDispose();
    }
  }

  void _toggleOptions() {
    setState(() {
      _showOptions = !_showOptions;
    });
  }

  void _showChapterList() {
    if (_readerState.chapters.isEmpty) return;

    showDialog(
      context: context,
      builder: (context) => ChapterListDialog(
        chapters: _readerState.chapters,
        currentIndex: _readerState.currentChapterIndex,
        onChapterSelected: (index) async {
          await _readerState.goToChapter(index);
          if (mounted) {
            _scrollToChapterIndex(index);
          }
        },
      ),
    );
  }

  void _showThemeSettings() {
    showDialog(
      context: context,
      builder: (context) => ThemeSettingsDialog(
        currentTheme: _theme,
        fontSize: _fontSize,
        lineHeight: _lineHeight,
        onThemeChanged: (theme) => setState(() => _theme = theme),
        onFontSizeChanged: (size) => setState(() => _fontSize = size),
        onLineHeightChanged: (height) => setState(() => _lineHeight = height),
      ),
    );
  }

  Future<void> _scrollToChapterIndex(int index) async {
    final key = _chapterKeys[index];
    if (key == null) return;
    final context = key.currentContext;
    if (context == null) return;

    await Scrollable.ensureVisible(
      context,
      duration: const Duration(milliseconds: 250),
      curve: Curves.easeOut,
      alignment: 0,
    );

    await _scrollToChapterPosition(index);
  }

  Color _getBackgroundColor() {
    switch (_theme) {
      case ReadingTheme.light:
        return Colors.white;
      case ReadingTheme.sepia:
        return const Color(0xFFF5F0E1);
      case ReadingTheme.dark:
        return Colors.grey[900]!;
    }
  }

  GlobalKey _getChapterKey(int index) {
    return _chapterKeys.putIfAbsent(index, () => GlobalKey());
  }

  void _maybePrefetchNextPage(int index) {
    final nextIndex = index + 1;
    if (nextIndex < _readerState.chapters.length) {
      _readerState.getChapterContent(nextIndex);
    }
  }

  Future<void> _loadMoreChapters(ReaderState state, int index) async {
    await _readerState.preloadAround(index);
  }

  void _saveChapterProgress(int index, double localOffset) {
    _readerState.updateChapterProgress(index, localOffset);
  }

  void _updateChapterCharProgress(ReaderState state, int index) {
    final content = state.getCachedChapterContent(index);
    if (content == null || content.isEmpty) return;

    final listBox = _listKey.currentContext?.findRenderObject() as RenderBox?;
    if (listBox == null) return;

    final chapterContext = _chapterKeys[index]?.currentContext;
    if (chapterContext == null) return;

    final chapterBox = chapterContext.findRenderObject() as RenderBox?;
    if (chapterBox == null) return;

    final chapterTop = chapterBox.localToGlobal(Offset.zero, ancestor: listBox).dy;
    final chapterHeight = chapterBox.size.height;
    if (chapterHeight <= 0) return;

    final visibleBottom = _scrollController.offset + _scrollController.position.viewportDimension;
    final chapterVisibleBottom = (visibleBottom - chapterTop).clamp(0, chapterHeight);
    final progress = (chapterVisibleBottom / chapterHeight).clamp(0, 1.0);

    final totalChars = content.length;
    final estimatedChar = (totalChars * progress).round();

    state.updateChapterCharProgress(index, estimatedChar);
  }

  Future<void> _scrollToChapterPosition(int index) async {
    final target = _readerState.getChapterScrollPosition(index);
    if (target <= 0) return;

    final key = _chapterKeys[index];
    final context = key?.currentContext;
    if (context == null) return;
    final renderBox = context.findRenderObject() as RenderBox?;
    if (renderBox == null) return;

    final listBox = _listKey.currentContext?.findRenderObject() as RenderBox?;
    if (listBox == null) return;

    final offset = renderBox.localToGlobal(Offset.zero, ancestor: listBox).dy;
    final targetOffset = _scrollController.offset + offset + target;

    await _scrollController.animateTo(
      targetOffset,
      duration: const Duration(milliseconds: 200),
      curve: Curves.easeOut,
    );
  }

  void _updateCurrentChapterFromScroll(ReaderState state) {
    if (_isUpdatingCurrentChapter) return;

    final listBox = _listKey.currentContext?.findRenderObject() as RenderBox?;
    if (listBox == null) return;

    int currentIndex = state.currentChapterIndex;
    double closestDistance = double.infinity;

    for (int i = 0; i < state.chapters.length; i++) {
      final key = _chapterKeys[i];
      final context = key?.currentContext;
      if (context == null) continue;

      final renderBox = context.findRenderObject() as RenderBox?;
      if (renderBox == null) continue;

      final offset = renderBox.localToGlobal(Offset.zero, ancestor: listBox).dy;
      final distance = offset.abs();

      if (distance < closestDistance) {
        closestDistance = distance;
        currentIndex = i;
      }
    }

    if (currentIndex != state.currentChapterIndex) {
      _isUpdatingCurrentChapter = true;
      _readerState.goToChapter(currentIndex).whenComplete(() {
        _isUpdatingCurrentChapter = false;
      });
    }
  }

  Color _getTextColor() {
    switch (_theme) {
      case ReadingTheme.light:
        return Colors.black87;
      case ReadingTheme.sepia:
        return Colors.brown;
      case ReadingTheme.dark:
        return Colors.white;
    }
  }

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: _readerState,
      builder: (context, child) {
        final state = _readerState;
        
        return Scaffold(
          backgroundColor: _getBackgroundColor(),
          body: SafeArea(
            child: state.isLoading
                ? Center(
                    child: CircularProgressIndicator(
                      color: _getTextColor(),
                    ),
                  )
                : state.error != null
                    ? _buildErrorView(state)
                    : state.chapters.isEmpty
                        ? _buildNoContentView(state)
                        : _buildReaderView(state),
          ),
        );
      },
    );
  }

  Widget _buildErrorView(ReaderState state) {
    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            '错误信息',
            style: Theme.of(context).textTheme.titleLarge?.copyWith(
                  color: Colors.red,
                ),
          ),
          const SizedBox(height: 8),
          Text(
            state.error!,
            style: TextStyle(color: _getTextColor()),
          ),
          const SizedBox(height: 16),
          if (state.book != null) ...[
            Text(
              '书籍信息',
              style: Theme.of(context).textTheme.titleMedium?.copyWith(
                    color: _getTextColor(),
                  ),
            ),
            const SizedBox(height: 8),
            Text('书名: ${state.book!.name}', style: TextStyle(color: _getTextColor())),
            Text('作者: ${state.book!.author}', style: TextStyle(color: _getTextColor())),
            Text('书籍类型: ${state.book!.bookType}', style: TextStyle(color: _getTextColor())),
            Text('是否网络书籍: ${state.isNetworkBook}', style: TextStyle(color: _getTextColor())),
            Text('章节数量: ${state.chapters.length}', style: TextStyle(color: _getTextColor())),
            Text('当前章节索引: ${state.currentChapterIndex}', style: TextStyle(color: _getTextColor())),
          ],
        ],
      ),
    );
  }

  Widget _buildNoContentView(ReaderState state) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Text('暂无章节内容', style: TextStyle(color: _getTextColor())),
          const SizedBox(height: 16),
          if (state.book != null) ...[
            Text('书籍类型: ${state.book!.bookType}', style: TextStyle(color: _getTextColor())),
            Text('是否网络书籍: ${state.isNetworkBook}', style: TextStyle(color: _getTextColor())),
            Text('章节数量: ${state.chapters.length}', style: TextStyle(color: _getTextColor())),
          ],
        ],
      ),
    );
  }

  Widget _buildReaderView(ReaderState state) {
    return Stack(
      children: [
        Listener(
          onPointerDown: (details) {
            _lastTapTime = DateTime.now();
            _lastTapPosition = details.position;
          },
          onPointerUp: (details) {
            if (_lastTapTime != null && _lastTapPosition != null) {
              final duration = DateTime.now().difference(_lastTapTime!);
              final distance = (details.position - _lastTapPosition!).distance;
              if (duration.inMilliseconds < 300 && distance < 10) {
                _toggleOptions();
              }
            }
          },
          child: NotificationListener<ScrollNotification>(
            onNotification: (notification) {
              if (notification is ScrollUpdateNotification) {
                _updateCurrentChapterFromScroll(state);
              }
              if (notification is ScrollEndNotification) {
                _readerState.saveReadingProgress();
              }
              return false;
            },
            child: ListView.builder(
              key: _listKey,
              controller: _scrollController,
              itemCount: state.chapters.length,
              itemBuilder: (context, index) {
                return _buildChapterPage(state, index);
              },
            ),
          ),
        ),
        AnimatedPositioned(
          duration: const Duration(milliseconds: 200),
          top: _showOptions ? 0 : -kToolbarHeight - 40,
          left: 0,
          right: 0,
          child: _buildTopBar(state),
        ),
        AnimatedPositioned(
          duration: const Duration(milliseconds: 200),
          bottom: _showOptions ? 0 : -80,
          left: 0,
          right: 0,
          child: _buildBottomBar(state),
        ),
      ],
    );
  }

  Widget _buildChapterPage(ReaderState state, int index) {
    final chapter = state.chapters[index];
    final cachedContent = state.getCachedChapterContent(index);

    return NotificationListener<ScrollNotification>(
      onNotification: (notification) {
        if (notification is ScrollUpdateNotification && index == state.currentChapterIndex) {
          state.updateScrollPosition(notification.metrics.pixels);
          _updateChapterCharProgress(state, index);
        }
        if (notification is ScrollEndNotification) {
          _saveChapterProgress(index, notification.metrics.pixels);
        }
        return false;
      },
      child: Container(
        key: _getChapterKey(index),
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 24),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            _buildChapterTitle(chapter),
            const SizedBox(height: 24),
            if (cachedContent != null)
              _buildContent(cachedContent)
            else
              _buildChapterContentLoader(state, index),
            _buildChapterFooter(state, index),
          ],
        ),
      ),
    );
  }

  Widget _buildChapterContentLoader(ReaderState state, int index) {
    if (index == state.currentChapterIndex) {
      return FutureBuilder(
        future: _readerState.getChapterContent(index),
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            return Center(
              child: Padding(
                padding: const EdgeInsets.all(32),
                child: CircularProgressIndicator(color: _getTextColor()),
              ),
            );
          } else if (snapshot.hasError) {
            return Text(
              '加载失败: ${snapshot.error}',
              style: TextStyle(color: _getTextColor()),
            );
          } else if (snapshot.hasData) {
            return _buildContent(snapshot.data!);
          } else {
            return Text(
              '暂无内容',
              style: TextStyle(color: _getTextColor()),
            );
          }
        },
      );
    }

    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 24),
      child: Text(
        '章节内容加载中…',
        style: TextStyle(color: _getTextColor()),
      ),
    );
  }

  Widget _buildContent(String content) {
    final textColor = _getTextColor();
    final hasHtmlTags = content.contains('<p') || content.contains('<div') || content.contains('<br');

    if (hasHtmlTags) {
      return _buildHtmlContent(content);
    } else {
      return _buildPlainTextContent(content, textColor);
    }
  }

  Widget _buildChapterTitle(Chapter chapter) {
    return Text(
      chapter.title,
      style: TextStyle(
        fontSize: _fontSize + 6,
        fontWeight: FontWeight.bold,
        color: _getTextColor(),
        height: _lineHeight,
      ),
    );
  }

  Widget _buildChapterFooter(ReaderState state, int index) {
    final isLast = index == state.chapters.length - 1;

    if (isLast) {
      return Padding(
        padding: const EdgeInsets.only(top: 32, bottom: 16),
        child: Text(
          '已经是最后一章',
          style: TextStyle(color: _getTextColor(), fontSize: 12),
          textAlign: TextAlign.center,
        ),
      );
    }

    return const SizedBox.shrink();
  }

  Widget _buildPlainTextContent(String content, Color textColor) {
    String processed = content
        .replaceAll(RegExp(r'\r\n'), '\n')
        .replaceAll(RegExp(r'\r'), '\n')
        .replaceAll(RegExp(r'\n{3,}'), '\n\n')
        .trim();
    
    List<String> paragraphs = processed.split(RegExp(r'\n{2,}'));
    
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: paragraphs.map((paragraph) {
        String trimmed = paragraph.trim();
        if (trimmed.isEmpty) return const SizedBox.shrink();
        
        return Padding(
          padding: const EdgeInsets.symmetric(vertical: 8),
          child: Text.rich(
            TextSpan(
              children: _buildTextSpans(trimmed, textColor),
            ),
            style: TextStyle(
              fontSize: _fontSize,
              height: _lineHeight,
              color: textColor,
              letterSpacing: 0.3,
            ),
            strutStyle: StrutStyle(
              fontSize: _fontSize,
              height: _lineHeight,
              leading: 0.2,
              forceStrutHeight: true,
            ),
          ),
        );
      }).toList(),
    );
  }

  List<TextSpan> _buildTextSpans(String text, Color textColor) {
    List<TextSpan> spans = [];
    StringBuffer buffer = StringBuffer();
    bool isFirstChar = true;
    
    for (int i = 0; i < text.length; i++) {
      String char = text[i];
      
      if (isFirstChar && char.trim().isNotEmpty) {
        spans.add(const TextSpan(text: '　　'));
        isFirstChar = false;
      }
      
      buffer.write(char);
    }
    
    if (buffer.isNotEmpty) {
      spans.add(TextSpan(
        text: buffer.toString(),
        style: TextStyle(
          color: textColor,
        ),
      ));
    }
    
    return spans;
  }

  String _preprocessContent(String content) {
    String processed = content;
    
    if (!processed.contains('<p') && !processed.contains('<div')) {
      processed = processed
          .replaceAll(RegExp(r'\r\n'), '\n')
          .replaceAll(RegExp(r'\r'), '\n')
          .replaceAll(RegExp(r'\n{3,}'), '\n\n')
          .trim();
      
      List<String> paragraphs = processed.split(RegExp(r'\n{2,}'));
      
      if (paragraphs.length > 1) {
        processed = paragraphs.map((p) {
          String trimmed = p.trim();
          if (trimmed.isNotEmpty) {
            return '<p>$trimmed</p>';
          }
          return '';
        }).join('');
      } else {
        processed = processed.replaceAll('\n', '<br/>');
      }
    }
    
    return processed;
  }

  Widget _buildHtmlContent(String htmlContent) {
    final textColor = _getTextColor();
    final processedContent = _preprocessContent(htmlContent);

    return HtmlWidget(
      processedContent,
      textStyle: TextStyle(
        fontSize: _fontSize,
        height: _lineHeight,
        color: textColor,
        letterSpacing: 0.5,
      ),
      customStylesBuilder: (element) {
        if (element.localName == 'p') {
          return {
            'margin': '16px 0',
            'text-indent': '2em',
            'text-align': 'justify',
            'line-height': '${_lineHeight.toString()}',
            'word-spacing': '2px',
          };
        } else if (element.localName == 'div') {
          return {
            'margin': '16px 0',
            'text-align': 'justify',
            'line-height': '${_lineHeight.toString()}',
          };
        } else if (element.localName == 'br') {
          return {
            'display': 'block',
            'height': '8px',
          };
        }
        return null;
      },
    );
  }

  @override
  void didUpdateWidget(covariant ReaderPage oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.bookId != widget.bookId) {
      _readerState.saveAndDispose();
      _readerState = ReaderState(bookId: widget.bookId);
      _readerState.initialize();
    }
  }

  Widget _buildTopBar(ReaderState state) {
    return Container(
      color: _theme == ReadingTheme.dark
          ? Colors.grey[800]
          : Colors.white.withOpacity(0.95),
      child: SafeArea(
        child: Row(
          children: [
            IconButton(
              icon: Icon(Icons.arrow_back, color: _getTextColor()),
              onPressed: () => Navigator.pop(context),
            ),
            Expanded(
              child: Text(
                state.currentChapter?.title ?? state.book?.name ?? '阅读器',
                style: TextStyle(
                  fontSize: 16,
                  fontWeight: FontWeight.bold,
                  color: _getTextColor(),
                ),
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
              ),
            ),
            if (state.currentChapter != null)
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 16),
                child: Text(
                  '${state.currentChapterIndex + 1}/${state.chapters.length}',
                  style: TextStyle(color: _getTextColor()),
                ),
              ),
          ],
        ),
      ),
    );
  }

  Widget _buildBottomBar(ReaderState state) {
    return Container(
      color: _theme == ReadingTheme.dark
          ? Colors.grey[800]
          : Colors.white.withOpacity(0.95),
      padding: const EdgeInsets.symmetric(vertical: 8, horizontal: 16),
      child: SafeArea(
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceAround,
          children: [
            IconButton(
              icon: const Icon(Icons.menu_book),
              color: _getTextColor(),
              onPressed: _showChapterList,
              tooltip: '目录',
            ),
            IconButton(
              icon: const Icon(Icons.palette),
              color: _getTextColor(),
              onPressed: _showThemeSettings,
              tooltip: '设置',
            ),
          ],
        ),
      ),
    );
  }
}
