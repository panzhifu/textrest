import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:textrest/src/app/features/setting/provider/book_source_controller.dart';
import 'package:textrest/src/rust/models/book_source.dart';
import 'package:textrest/src/rust/models/rules.dart';

class BookSourceEditPage extends ConsumerStatefulWidget {
  final BookSource source;

  const BookSourceEditPage({super.key, required this.source});

  @override
  ConsumerState<BookSourceEditPage> createState() => _BookSourceEditPageState();
}

class _BookSourceEditPageState extends ConsumerState<BookSourceEditPage> {
  late final TextEditingController _nameController;
  late final TextEditingController _urlController;
  late final TextEditingController _groupController;
  late final TextEditingController _commentController;
  late final TextEditingController _searchUrlController;
  late final TextEditingController _exploreUrlController;

  late final TextEditingController _searchListController;
  late final TextEditingController _searchNameController;
  late final TextEditingController _searchAuthorController;
  late final TextEditingController _searchCoverController;
  late final TextEditingController _searchIntroController;
  late final TextEditingController _searchUrlRuleController;

  late final TextEditingController _bookNameController;
  late final TextEditingController _bookAuthorController;
  late final TextEditingController _bookIntroController;
  late final TextEditingController _bookCoverController;
  late final TextEditingController _bookTocController;

  late final TextEditingController _tocListController;
  late final TextEditingController _tocNameController;
  late final TextEditingController _tocUrlController;

  late final TextEditingController _contentController;
  late final TextEditingController _nextContentController;
  late final TextEditingController _prevContentController;

  late final TextEditingController _exploreListController;
  late final TextEditingController _exploreNameController;
  late final TextEditingController _exploreUrlRuleController;

  late final TextEditingController _reviewListController;
  late final TextEditingController _reviewContentController;

  bool _enabled = false;

  @override
  void initState() {
    super.initState();
    final source = widget.source;
    _nameController = TextEditingController(text: source.bookSourceName);
    _urlController = TextEditingController(text: source.bookSourceUrl);
    _groupController = TextEditingController(text: source.bookSourceGroup ?? '');
    _commentController = TextEditingController(text: source.bookSourceComment ?? '');
    _searchUrlController = TextEditingController(text: source.searchUrl ?? '');
    _exploreUrlController = TextEditingController(text: source.exploreUrl ?? '');

    _searchListController = TextEditingController(text: source.searchRule?.list ?? '');
    _searchNameController = TextEditingController(text: source.searchRule?.name ?? '');
    _searchAuthorController = TextEditingController(text: source.searchRule?.author ?? '');
    _searchCoverController = TextEditingController(text: source.searchRule?.cover ?? '');
    _searchIntroController = TextEditingController(text: source.searchRule?.intro ?? '');
    _searchUrlRuleController = TextEditingController(text: source.searchRule?.url ?? '');

    _bookNameController = TextEditingController(text: source.bookInfoRule?.name ?? '');
    _bookAuthorController = TextEditingController(text: source.bookInfoRule?.author ?? '');
    _bookIntroController = TextEditingController(text: source.bookInfoRule?.intro ?? '');
    _bookCoverController = TextEditingController(text: source.bookInfoRule?.coverUrl ?? '');
    _bookTocController = TextEditingController(text: source.bookInfoRule?.tocUrl ?? '');

    _tocListController = TextEditingController(text: source.tocRule?.chapterList ?? '');
    _tocNameController = TextEditingController(text: source.tocRule?.chapterName ?? '');
    _tocUrlController = TextEditingController(text: source.tocRule?.chapterUrl ?? '');

    _contentController = TextEditingController(text: source.contentRule?.content ?? '');
    _nextContentController = TextEditingController(text: source.contentRule?.nextContent ?? '');
    _prevContentController = TextEditingController(text: source.contentRule?.prevContent ?? '');

    _exploreListController = TextEditingController(text: source.ruleExplore?.list ?? '');
    _exploreNameController = TextEditingController(text: source.ruleExplore?.name ?? '');
    _exploreUrlRuleController = TextEditingController(text: source.ruleExplore?.url ?? '');

    _reviewListController = TextEditingController(text: source.reviewRule?.list ?? '');
    _reviewContentController = TextEditingController(text: source.reviewRule?.content ?? '');

    _enabled = source.enabled;
  }

  @override
  void dispose() {
    _nameController.dispose();
    _urlController.dispose();
    _groupController.dispose();
    _commentController.dispose();
    _searchUrlController.dispose();
    _exploreUrlController.dispose();
    _searchListController.dispose();
    _searchNameController.dispose();
    _searchAuthorController.dispose();
    _searchCoverController.dispose();
    _searchIntroController.dispose();
    _searchUrlRuleController.dispose();
    _bookNameController.dispose();
    _bookAuthorController.dispose();
    _bookIntroController.dispose();
    _bookCoverController.dispose();
    _bookTocController.dispose();
    _tocListController.dispose();
    _tocNameController.dispose();
    _tocUrlController.dispose();
    _contentController.dispose();
    _nextContentController.dispose();
    _prevContentController.dispose();
    _exploreListController.dispose();
    _exploreNameController.dispose();
    _exploreUrlRuleController.dispose();
    _reviewListController.dispose();
    _reviewContentController.dispose();
    super.dispose();
  }

  Future<void> _save() async {
    final controller = ref.read(bookSourceProvider.notifier);
    final updated = BookSource(
      bookSourceUrl: _urlController.text.trim(),
      bookSourceName: _nameController.text.trim(),
      bookSourceGroup: _groupController.text.trim().isEmpty ? null : _groupController.text.trim(),
      bookSourceType: widget.source.bookSourceType,
      bookUrlPattern: widget.source.bookUrlPattern,
      customOrder: widget.source.customOrder,
      enabled: _enabled,
      enabledExplore: widget.source.enabledExplore,
      jsLib: widget.source.jsLib,
      enabledCookieJar: widget.source.enabledCookieJar,
      concurrentRate: widget.source.concurrentRate,
      header: widget.source.header,
      loginUrl: widget.source.loginUrl,
      loginUi: widget.source.loginUi,
      loginCheckJs: widget.source.loginCheckJs,
      coverDecodeJs: widget.source.coverDecodeJs,
      bookSourceComment: _commentController.text.trim().isEmpty
          ? null
          : _commentController.text.trim(),
      variableComment: widget.source.variableComment,
      lastUpdateTime: widget.source.lastUpdateTime,
      respondTime: widget.source.respondTime,
      weight: widget.source.weight,
      exploreUrl: _exploreUrlController.text.trim().isEmpty
          ? null
          : _exploreUrlController.text.trim(),
      exploreScreen: widget.source.exploreScreen,
      ruleExplore: _buildExploreRule(),
      searchUrl: _searchUrlController.text.trim().isEmpty
          ? null
          : _searchUrlController.text.trim(),
      searchRule: _buildSearchRule(),
      bookInfoRule: _buildBookInfoRule(),
      tocRule: _buildTocRule(),
      contentRule: _buildContentRule(),
      reviewRule: _buildReviewRule(),
    );

    await controller.updateSource(updated);
    if (!mounted) return;
    Navigator.pop(context);
  }

  ExploreRule? _buildExploreRule() {
    if (_exploreListController.text.trim().isEmpty &&
        _exploreNameController.text.trim().isEmpty &&
        _exploreUrlRuleController.text.trim().isEmpty) {
      return widget.source.ruleExplore;
    }
    return ExploreRule(
      init: widget.source.ruleExplore?.init,
      list: _exploreListController.text.trim().isEmpty
          ? null
          : _exploreListController.text.trim(),
      name: _exploreNameController.text.trim().isEmpty
          ? null
          : _exploreNameController.text.trim(),
      url: _exploreUrlRuleController.text.trim().isEmpty
          ? null
          : _exploreUrlRuleController.text.trim(),
      cover: widget.source.ruleExplore?.cover,
      author: widget.source.ruleExplore?.author,
      intro: widget.source.ruleExplore?.intro,
      kind: widget.source.ruleExplore?.kind,
      lastChapter: widget.source.ruleExplore?.lastChapter,
      updateTime: widget.source.ruleExplore?.updateTime,
      wordCount: widget.source.ruleExplore?.wordCount,
      nextPage: widget.source.ruleExplore?.nextPage,
      sort: widget.source.ruleExplore?.sort,
      filter: widget.source.ruleExplore?.filter,
      search: widget.source.ruleExplore?.search,
      comment: widget.source.ruleExplore?.comment,
      chapter: widget.source.ruleExplore?.chapter,
      content: widget.source.ruleExplore?.content,
    );
  }

  SearchRule? _buildSearchRule() {
    if (_searchListController.text.trim().isEmpty &&
        _searchNameController.text.trim().isEmpty &&
        _searchUrlRuleController.text.trim().isEmpty) {
      return widget.source.searchRule;
    }
    return SearchRule(
      init: widget.source.searchRule?.init,
      list: _searchListController.text.trim().isEmpty ? null : _searchListController.text.trim(),
      name: _searchNameController.text.trim().isEmpty ? null : _searchNameController.text.trim(),
      url: _searchUrlRuleController.text.trim().isEmpty ? null : _searchUrlRuleController.text.trim(),
      cover: _searchCoverController.text.trim().isEmpty ? null : _searchCoverController.text.trim(),
      author: _searchAuthorController.text.trim().isEmpty ? null : _searchAuthorController.text.trim(),
      intro: _searchIntroController.text.trim().isEmpty ? null : _searchIntroController.text.trim(),
      kind: widget.source.searchRule?.kind,
      lastChapter: widget.source.searchRule?.lastChapter,
      updateTime: widget.source.searchRule?.updateTime,
      wordCount: widget.source.searchRule?.wordCount,
      nextPage: widget.source.searchRule?.nextPage,
    );
  }

  BookInfoRule? _buildBookInfoRule() {
    if (_bookNameController.text.trim().isEmpty &&
        _bookAuthorController.text.trim().isEmpty &&
        _bookIntroController.text.trim().isEmpty) {
      return widget.source.bookInfoRule;
    }
    return BookInfoRule(
      init: widget.source.bookInfoRule?.init,
      name: _bookNameController.text.trim().isEmpty ? null : _bookNameController.text.trim(),
      author: _bookAuthorController.text.trim().isEmpty ? null : _bookAuthorController.text.trim(),
      intro: _bookIntroController.text.trim().isEmpty ? null : _bookIntroController.text.trim(),
      kind: widget.source.bookInfoRule?.kind,
      lastChapter: widget.source.bookInfoRule?.lastChapter,
      updateTime: widget.source.bookInfoRule?.updateTime,
      coverUrl: _bookCoverController.text.trim().isEmpty ? null : _bookCoverController.text.trim(),
      tocUrl: _bookTocController.text.trim().isEmpty ? null : _bookTocController.text.trim(),
      wordCount: widget.source.bookInfoRule?.wordCount,
      canReName: widget.source.bookInfoRule?.canReName,
      downloadUrls: widget.source.bookInfoRule?.downloadUrls,
    );
  }

  TocRule? _buildTocRule() {
    if (_tocListController.text.trim().isEmpty &&
        _tocNameController.text.trim().isEmpty &&
        _tocUrlController.text.trim().isEmpty) {
      return widget.source.tocRule;
    }
    return TocRule(
      preUpdateJs: widget.source.tocRule?.preUpdateJs,
      chapterList: _tocListController.text.trim().isEmpty ? null : _tocListController.text.trim(),
      chapterName: _tocNameController.text.trim().isEmpty ? null : _tocNameController.text.trim(),
      chapterUrl: _tocUrlController.text.trim().isEmpty ? null : _tocUrlController.text.trim(),
      formatJs: widget.source.tocRule?.formatJs,
      isVolume: widget.source.tocRule?.isVolume,
      isVip: widget.source.tocRule?.isVip,
      isPay: widget.source.tocRule?.isPay,
      updateTime: widget.source.tocRule?.updateTime,
      nextTocUrl: widget.source.tocRule?.nextTocUrl,
    );
  }

  ContentRule? _buildContentRule() {
    if (_contentController.text.trim().isEmpty &&
        _nextContentController.text.trim().isEmpty &&
        _prevContentController.text.trim().isEmpty) {
      return widget.source.contentRule;
    }
    return ContentRule(
      init: widget.source.contentRule?.init,
      content: _contentController.text.trim().isEmpty ? null : _contentController.text.trim(),
      nextContent:
          _nextContentController.text.trim().isEmpty ? null : _nextContentController.text.trim(),
      prevContent:
          _prevContentController.text.trim().isEmpty ? null : _prevContentController.text.trim(),
      refreshContent: widget.source.contentRule?.refreshContent,
      replaceRegex: widget.source.contentRule?.replaceRegex,
      removeRegex: widget.source.contentRule?.removeRegex,
      cssSelector: widget.source.contentRule?.cssSelector,
      isWebView: widget.source.contentRule?.isWebView,
      js: widget.source.contentRule?.js,
      encode: widget.source.contentRule?.encode,
    );
  }

  ReviewRule? _buildReviewRule() {
    if (_reviewListController.text.trim().isEmpty &&
        _reviewContentController.text.trim().isEmpty) {
      return widget.source.reviewRule;
    }
    return ReviewRule(
      init: widget.source.reviewRule?.init,
      list: _reviewListController.text.trim().isEmpty ? null : _reviewListController.text.trim(),
      name: widget.source.reviewRule?.name,
      content: _reviewContentController.text.trim().isEmpty ? null : _reviewContentController.text.trim(),
      time: widget.source.reviewRule?.time,
      rating: widget.source.reviewRule?.rating,
      nextPage: widget.source.reviewRule?.nextPage,
    );
  }

  Future<void> _delete() async {
    final controller = ref.read(bookSourceProvider.notifier);
    await controller.deleteSource(widget.source.bookSourceUrl);
    if (!mounted) return;
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('编辑书源'),
        actions: [
          IconButton(
            icon: const Icon(Icons.save),
            onPressed: _save,
          ),
        ],
      ),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          _SectionCard(
            title: '基础信息',
            child: Column(
              children: [
                _textField(_nameController, '书源名称'),
                const SizedBox(height: 12),
                _textField(_urlController, '书源地址', readOnly: true),
                const SizedBox(height: 12),
                _textField(_groupController, '分组'),
                const SizedBox(height: 12),
                _textField(_commentController, '备注', maxLines: 3),
                const SizedBox(height: 12),
                SwitchListTile(
                  contentPadding: EdgeInsets.zero,
                  title: const Text('启用书源'),
                  value: _enabled,
                  onChanged: (value) => setState(() => _enabled = value),
                ),
              ],
            ),
          ),
          const SizedBox(height: 16),
          _SectionCard(
            title: '搜索规则',
            child: Column(
              children: [
                _textField(_searchUrlController, '搜索链接'),
                const SizedBox(height: 12),
                _textField(_searchListController, '列表规则'),
                const SizedBox(height: 12),
                _textField(_searchNameController, '名称规则'),
                const SizedBox(height: 12),
                _textField(_searchAuthorController, '作者规则'),
                const SizedBox(height: 12),
                _textField(_searchCoverController, '封面规则'),
                const SizedBox(height: 12),
                _textField(_searchIntroController, '简介规则', maxLines: 2),
                const SizedBox(height: 12),
                _textField(_searchUrlRuleController, '详情链接规则'),
              ],
            ),
          ),
          const SizedBox(height: 16),
          _SectionCard(
            title: '书籍信息规则',
            child: Column(
              children: [
                _textField(_bookNameController, '书名规则'),
                const SizedBox(height: 12),
                _textField(_bookAuthorController, '作者规则'),
                const SizedBox(height: 12),
                _textField(_bookIntroController, '简介规则', maxLines: 3),
                const SizedBox(height: 12),
                _textField(_bookCoverController, '封面规则'),
                const SizedBox(height: 12),
                _textField(_bookTocController, '目录链接规则'),
              ],
            ),
          ),
          const SizedBox(height: 16),
          _SectionCard(
            title: '目录规则',
            child: Column(
              children: [
                _textField(_tocListController, '章节列表规则'),
                const SizedBox(height: 12),
                _textField(_tocNameController, '章节标题规则'),
                const SizedBox(height: 12),
                _textField(_tocUrlController, '章节链接规则'),
              ],
            ),
          ),
          const SizedBox(height: 16),
          _SectionCard(
            title: '正文规则',
            child: Column(
              children: [
                _textField(_contentController, '正文规则', maxLines: 3),
                const SizedBox(height: 12),
                _textField(_nextContentController, '下一页规则'),
                const SizedBox(height: 12),
                _textField(_prevContentController, '上一页规则'),
              ],
            ),
          ),
          const SizedBox(height: 16),
          _SectionCard(
            title: '发现规则',
            child: Column(
              children: [
                _textField(_exploreUrlController, '发现链接'),
                const SizedBox(height: 12),
                _textField(_exploreListController, '列表规则'),
                const SizedBox(height: 12),
                _textField(_exploreNameController, '名称规则'),
                const SizedBox(height: 12),
                _textField(_exploreUrlRuleController, '详情链接规则'),
              ],
            ),
          ),
          const SizedBox(height: 16),
          _SectionCard(
            title: '评论规则',
            child: Column(
              children: [
                _textField(_reviewListController, '评论列表规则'),
                const SizedBox(height: 12),
                _textField(_reviewContentController, '评论内容规则', maxLines: 3),
              ],
            ),
          ),
          const SizedBox(height: 16),
          OutlinedButton.icon(
            onPressed: _delete,
            icon: const Icon(Icons.delete_outline),
            label: const Text('删除书源'),
          ),
        ],
      ),
    );
  }

  Widget _textField(TextEditingController controller, String label, {int maxLines = 1, bool readOnly = false}) {
    return TextField(
      controller: controller,
      maxLines: maxLines,
      readOnly: readOnly,
      decoration: InputDecoration(
        labelText: label,
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
        ),
      ),
    );
  }
}

class _SectionCard extends StatelessWidget {
  final String title;
  final Widget child;

  const _SectionCard({required this.title, required this.child});

  @override
  Widget build(BuildContext context) {
    return Card(
      elevation: 0,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(16),
        side: BorderSide(color: Theme.of(context).dividerColor.withOpacity(0.2)),
      ),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              title,
              style: Theme.of(context).textTheme.titleSmall?.copyWith(
                    fontWeight: FontWeight.w600,
                  ),
            ),
            const SizedBox(height: 12),
            child,
          ],
        ),
      ),
    );
  }
}
