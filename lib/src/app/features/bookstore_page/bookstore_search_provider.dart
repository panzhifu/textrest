import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:textrest/src/rust/api/web_book.dart';
import 'package:textrest/src/rust/models/book.dart';

class BookstoreSearchState {
  const BookstoreSearchState({
    this.results = const <Book>[],
    this.isLoading = false,
    this.errorMessage,
  });

  final List<Book> results;
  final bool isLoading;
  final String? errorMessage;

  BookstoreSearchState copyWith({
    List<Book>? results,
    bool? isLoading,
    String? errorMessage,
  }) {
    return BookstoreSearchState(
      results: results ?? this.results,
      isLoading: isLoading ?? this.isLoading,
      errorMessage: errorMessage,
    );
  }
}

class BookstoreSearchNotifier extends StateNotifier<BookstoreSearchState> {
  BookstoreSearchNotifier(this.keyword) : super(const BookstoreSearchState());

  final String keyword;
  WebBookApi? _api;

  Future<void> search() async {
    if (keyword.trim().isEmpty) {
      state = state.copyWith(results: const <Book>[], isLoading: false);
      return;
    }

    state = state.copyWith(isLoading: true, errorMessage: null);
    try {
      _api ??= await WebBookApi.newInstance();
      final books = await _api!.search(keyword: keyword);
      state = state.copyWith(results: books, isLoading: false);
    } catch (error) {
      state = state.copyWith(
        results: const <Book>[],
        isLoading: false,
        errorMessage: error.toString(),
      );
    }
  }
}

final bookstoreSearchProvider = StateNotifierProvider.autoDispose
    .family<BookstoreSearchNotifier, BookstoreSearchState, String>(
  (ref, keyword) => BookstoreSearchNotifier(keyword),
);
