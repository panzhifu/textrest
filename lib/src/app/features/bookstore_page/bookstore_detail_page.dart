import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:textrest/src/app/features/bookshelf_page/bookcard.dart';
import 'package:textrest/src/app/features/bookstore_page/bookstore_detail_provider.dart';
import 'package:textrest/src/rust/models/book.dart';

class BookstoreDetailPage extends ConsumerStatefulWidget {
  final Book book;

  const BookstoreDetailPage({super.key, required this.book});

  @override
  ConsumerState<BookstoreDetailPage> createState() => _BookstoreDetailPageState();
}

class _BookstoreDetailPageState extends ConsumerState<BookstoreDetailPage> {
  @override
  void initState() {
    super.initState();
    Future.microtask(
      () => ref.read(bookstoreDetailProvider(widget.book).notifier).load(),
    );
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(bookstoreDetailProvider(widget.book));
    final book = state.book;
    final latestChapter = book.latestChapterTitle ?? '暂无最新章节';
    final intro = book.intro?.trim().isNotEmpty == true ? book.intro! : '暂无简介';

    return Scaffold(
      appBar: AppBar(title: const Text('书籍详情')),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          Center(
            child: SizedBox(
              width: 140,
              height: 240,
              child: BookCard(book: book),
            ),
          ),
          const SizedBox(height: 16),
          _SectionCard(
            title: '最新章节',
            child: Text(
              latestChapter,
              style: Theme.of(context).textTheme.bodyMedium,
            ),
          ),
          const SizedBox(height: 12),
          _SectionCard(
            title: '简介',
            child: Text(
              intro,
              style: Theme.of(context).textTheme.bodyMedium?.copyWith(height: 1.6),
            ),
          ),
          const SizedBox(height: 12),
          _SectionCard(
            title: '目录地址',
            child: state.isLoading
                ? const Text('加载中...')
                : state.tocUrls.isEmpty
                    ? const Text('暂无目录地址')
                    : Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: state.tocUrls
                            .map(
                              (url) => Padding(
                                padding: const EdgeInsets.only(bottom: 6),
                                child: Text(url, style: Theme.of(context).textTheme.bodySmall),
                              ),
                            )
                            .toList(),
                      ),
          ),
          if (state.errorMessage != null && state.errorMessage!.isNotEmpty)
            Padding(
              padding: const EdgeInsets.only(top: 12),
              child: Text(
                '加载失败: ${state.errorMessage}',
                style: TextStyle(color: Theme.of(context).colorScheme.error),
              ),
            ),
          const SizedBox(height: 16),
          Row(
            children: [
              Expanded(
                child: OutlinedButton(
                  onPressed: state.isAdding
                      ? null
                      : () async {
                          final notifier = ref.read(
                            bookstoreDetailProvider(widget.book).notifier,
                          );
                          final addedBook = await notifier.addToBookshelf();
                          if (!mounted) return;
                          if (addedBook != null) {
                            ScaffoldMessenger.of(context).showSnackBar(
                              const SnackBar(content: Text('已加入书架')),
                            );
                          } else if (state.errorMessage != null &&
                              state.errorMessage!.isNotEmpty) {
                            ScaffoldMessenger.of(context).showSnackBar(
                              SnackBar(content: Text('加入失败: ${state.errorMessage}')),
                            );
                          }
                        },
                  child: state.isAdding
                      ? const SizedBox(
                          width: 16,
                          height: 16,
                          child: CircularProgressIndicator(strokeWidth: 2),
                        )
                      : const Text('加入书架'),
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: ElevatedButton(
                  onPressed: () {
                    // TODO: 阅读
                  },
                  child: const Text('阅读'),
                ),
              ),
            ],
          ),
        ],
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
        side: BorderSide(color: Theme.of(context).dividerColor.withValues(alpha: 0.2)),
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
            const SizedBox(height: 8),
            child,
          ],
        ),
      ),
    );
  }
}
