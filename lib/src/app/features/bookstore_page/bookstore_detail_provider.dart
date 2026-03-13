import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:textrest/src/rust/api/web_book.dart';
import 'package:textrest/src/rust/models/book.dart';
import 'package:textrest/src/rust/models/book_source.dart';

class BookstoreDetailState {
  const BookstoreDetailState({
    required this.book,
    this.tocUrls = const <String>[],
    this.isLoading = false,
    this.isAdding = false,
    this.errorMessage,
  });

  final Book book;
  final List<String> tocUrls;
  final bool isLoading;
  final bool isAdding;
  final String? errorMessage;

  BookstoreDetailState copyWith({
    Book? book,
    List<String>? tocUrls,
    bool? isLoading,
    bool? isAdding,
    String? errorMessage,
  }) {
    return BookstoreDetailState(
      book: book ?? this.book,
      tocUrls: tocUrls ?? this.tocUrls,
      isLoading: isLoading ?? this.isLoading,
      isAdding: isAdding ?? this.isAdding,
      errorMessage: errorMessage ?? this.errorMessage,
    );
  }
}

class BookstoreDetailNotifier extends StateNotifier<BookstoreDetailState> {
  BookstoreDetailNotifier(this.baseBook)
      : super(BookstoreDetailState(book: baseBook));

  final Book baseBook;
  WebBookApi? _api;

  Future<void> load() async {
    state = state.copyWith(isLoading: true, errorMessage: null);
    try {
      _api ??= await WebBookApi.newInstance();
      final bookSource = await _resolveBookSource();
      if (bookSource == null) {
        state = state.copyWith(isLoading: false);
        return;
      }
      final updatedBook = await _api!.getBookInfoFromSearch(
        bookSource: bookSource,
        bookUrl: baseBook.bookId,
        baseBook: baseBook,
      );
      final tocUrls = await _api!.getBookTocUrlsFromSearch(
        bookSource: bookSource,
        bookUrl: baseBook.bookId,
        baseBook: baseBook,
      );
      state = state.copyWith(
        book: updatedBook,
        tocUrls: tocUrls,
        isLoading: false,
      );
    } catch (error) {
      state = state.copyWith(
        isLoading: false,
        errorMessage: error.toString(),
      );
    }
  }

  Future<Book?> addToBookshelf() async {
    state = state.copyWith(isAdding: true, errorMessage: null);
    try {
      _api ??= await WebBookApi.newInstance();
      final bookSource = await _resolveBookSource();
      if (bookSource == null) {
        state = state.copyWith(isAdding: false);
        return null;
      }
      final addedBook = await _api!.addNetworkBook(
        bookSource: bookSource,
        bookUrl: baseBook.bookId,
      );
      state = state.copyWith(
        book: addedBook,
        isAdding: false,
      );
      return addedBook;
    } catch (error) {
      state = state.copyWith(
        isAdding: false,
        errorMessage: error.toString(),
      );
      return null;
    }
  }

  Future<BookSource?> _resolveBookSource() async {
    final origin = baseBook.origin?.trim();
    if (origin == null || origin.isEmpty) {
      return null;
    }
    final sources = await _api!.loadBookSources();
    for (final source in sources) {
      if (source.bookSourceUrl == origin || source.bookSourceName == origin) {
        return source;
      }
    }
    return sources.isNotEmpty ? sources.first : null;
  }
}

final bookstoreDetailProvider = StateNotifierProvider.autoDispose
    .family<BookstoreDetailNotifier, BookstoreDetailState, Book>(
  (ref, book) => BookstoreDetailNotifier(book),
);
