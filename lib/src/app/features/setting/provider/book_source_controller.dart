import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:textrest/src/rust/api/book_source.dart';
import 'package:textrest/src/rust/api/data_base.dart';
import 'package:textrest/src/rust/models/book_source.dart';

final bookSourceProvider = StateNotifierProvider<BookSourceController, BookSourceState>(
  (ref) => BookSourceController()..loadSources(),
);

class BookSourceState {
  final List<BookSource> sources;
  final Set<int> selected;
  final bool isLoading;
  final String keyword;
  final String? error;

  const BookSourceState({
    required this.sources,
    required this.selected,
    required this.isLoading,
    required this.keyword,
    this.error,
  });

  List<BookSource> get filtered {
    if (keyword.trim().isEmpty) return sources;
    return sources.where((item) => item.bookSourceName.contains(keyword)).toList();
  }

  BookSourceState copyWith({
    List<BookSource>? sources,
    Set<int>? selected,
    bool? isLoading,
    String? keyword,
    String? error,
  }) {
    return BookSourceState(
      sources: sources ?? this.sources,
      selected: selected ?? this.selected,
      isLoading: isLoading ?? this.isLoading,
      keyword: keyword ?? this.keyword,
      error: error,
    );
  }

  static BookSourceState initial() => const BookSourceState(
        sources: [],
        selected: {},
        isLoading: true,
        keyword: '',
      );
}

class BookSourceController extends StateNotifier<BookSourceState> {
  BookSourceController() : super(BookSourceState.initial());

  BookSourceApi? _api;

  Future<void> _ensureApi() async {
    try {
      await initDatabase();
    } catch (_) {
      // 已初始化时忽略
    }
    _api ??= await BookSourceApi.newInstance();
  }

  Future<void> loadSources() async {
    state = state.copyWith(isLoading: true, error: null);
    try {
      await _ensureApi();
      final sources = await _api!.listAllSources();
      state = state.copyWith(sources: sources, selected: {}, isLoading: false);
    } catch (error) {
      state = state.copyWith(isLoading: false, error: '$error');
    }
  }

  void updateKeyword(String value) {
    state = state.copyWith(keyword: value.trim(), selected: {});
  }

  void clearKeyword() {
    state = state.copyWith(keyword: '', selected: {});
  }

  void toggleSelect(int index) {
    final selected = {...state.selected};
    if (selected.contains(index)) {
      selected.remove(index);
    } else {
      selected.add(index);
    }
    state = state.copyWith(selected: selected);
  }

  void toggleSelectAll(bool value) {
    if (!value) {
      state = state.copyWith(selected: {});
      return;
    }
    final indices = List.generate(state.filtered.length, (i) => i).toSet();
    state = state.copyWith(selected: indices);
  }

  Future<void> toggleEnabledForSelected(bool enabled) async {
    if (state.selected.isEmpty) return;
    try {
      await _ensureApi();
      final filtered = state.filtered;
      final updatedSources = [...state.sources];
      for (final index in state.selected) {
        final source = filtered[index];
        final updated = _copySource(source, enabled: enabled);
        await _api!.updateSource(source: updated);
        final originalIndex = updatedSources.indexOf(source);
        if (originalIndex != -1) {
          updatedSources[originalIndex] = updated;
        }
      }
      state = state.copyWith(sources: updatedSources, selected: {});
    } catch (error) {
      state = state.copyWith(error: '$error');
    }
  }

  Future<void> deleteSelected() async {
    if (state.selected.isEmpty) return;
    try {
      await _ensureApi();
      final filtered = state.filtered;
      final toRemove = state.selected.map((index) => filtered[index]).toSet();
      for (final source in toRemove) {
        await _api!.deleteSource(url: source.bookSourceUrl);
      }
      final updatedSources = [...state.sources]..removeWhere((item) => toRemove.contains(item));
      state = state.copyWith(sources: updatedSources, selected: {});
    } catch (error) {
      state = state.copyWith(error: '$error');
    }
  }

  Future<void> deleteSource(String url) async {
    try {
      await _ensureApi();
      await _api!.deleteSource(url: url);
      await loadSources();
    } catch (error) {
      state = state.copyWith(error: '$error');
    }
  }

  Future<void> toggleSingle(BookSource source, bool enabled) async {
    try {
      await _ensureApi();
      final updated = _copySource(source, enabled: enabled);
      await _api!.updateSource(source: updated);
      final updatedSources = [...state.sources];
      final index = updatedSources.indexOf(source);
      if (index != -1) {
        updatedSources[index] = updated;
      }
      state = state.copyWith(sources: updatedSources);
    } catch (error) {
      state = state.copyWith(error: '$error');
    }
  }

  Future<void> updateSource(BookSource updated) async {
    try {
      await _ensureApi();
      await _api!.updateSource(source: updated);
      final updatedSources = [...state.sources];
      final index = updatedSources.indexOf(updated);
      if (index != -1) {
        updatedSources[index] = updated;
      } else {
        updatedSources.add(updated);
      }
      state = state.copyWith(sources: updatedSources);
    } catch (error) {
      state = state.copyWith(error: '$error');
    }
  }

  Future<void> importFromUrl(String url) async {
    await _import(() => _api!.importFromUrl(url: url));
  }

  Future<void> importFromText(String text) async {
    await _import(() => _api!.importFromText(text: text));
  }

  Future<void> importFromFile(String filePath) async {
    await _import(() => _api!.importFromFile(filePath: filePath));
  }

  Future<void> _import(Future<List<BookSource>> Function() action) async {
    try {
      await _ensureApi();
      await action();
      await loadSources();
    } catch (error) {
      state = state.copyWith(error: '$error');
    }
  }

  BookSource _copySource(BookSource source, {required bool enabled}) {
    return BookSource(
      bookSourceUrl: source.bookSourceUrl,
      bookSourceName: source.bookSourceName,
      bookSourceGroup: source.bookSourceGroup,
      bookSourceType: source.bookSourceType,
      bookUrlPattern: source.bookUrlPattern,
      customOrder: source.customOrder,
      enabled: enabled,
      enabledExplore: source.enabledExplore,
      jsLib: source.jsLib,
      enabledCookieJar: source.enabledCookieJar,
      concurrentRate: source.concurrentRate,
      header: source.header,
      loginUrl: source.loginUrl,
      loginUi: source.loginUi,
      loginCheckJs: source.loginCheckJs,
      coverDecodeJs: source.coverDecodeJs,
      bookSourceComment: source.bookSourceComment,
      variableComment: source.variableComment,
      lastUpdateTime: source.lastUpdateTime,
      respondTime: source.respondTime,
      weight: source.weight,
      exploreUrl: source.exploreUrl,
      exploreScreen: source.exploreScreen,
      ruleExplore: source.ruleExplore,
      searchUrl: source.searchUrl,
      searchRule: source.searchRule,
      bookInfoRule: source.bookInfoRule,
      tocRule: source.tocRule,
      contentRule: source.contentRule,
      reviewRule: source.reviewRule,
    );
  }
}
