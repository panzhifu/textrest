import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:textrest/src/app/features/bookshelf_page/bookcard.dart';
import 'package:textrest/src/app/features/bookstore_page/bookstore_detail_page.dart';
import 'package:textrest/src/app/features/bookstore_page/bookstore_search_provider.dart';

class BookstoreSearchPage extends ConsumerStatefulWidget {
  final String keyword;

  const BookstoreSearchPage({super.key, required this.keyword});

  @override
  ConsumerState<BookstoreSearchPage> createState() => _BookstoreSearchPageState();
}

class _BookstoreSearchPageState extends ConsumerState<BookstoreSearchPage> {
  @override
  void initState() {
    super.initState();
    Future.microtask(() {
      ref.read(bookstoreSearchProvider(widget.keyword).notifier).search();
    });
  }

  @override
  Widget build(BuildContext context) {
    final isMobile = MediaQuery.of(context).size.width < 600;
    final gridDelegate = isMobile
        ? const SliverGridDelegateWithFixedCrossAxisCount(
            crossAxisCount: 3,
            crossAxisSpacing: 12,
            mainAxisSpacing: 16,
            childAspectRatio: 0.7,
          )
        : const SliverGridDelegateWithFixedCrossAxisCount(
            crossAxisCount: 5,
            crossAxisSpacing: 18,
            mainAxisSpacing: 20,
            childAspectRatio: 0.7,
          );

    final state = ref.watch(bookstoreSearchProvider(widget.keyword));

    return Scaffold(
      appBar: AppBar(title: const Text('搜索结果')),
      body: Builder(
        builder: (context) {
          if (state.isLoading) {
            return const Center(child: CircularProgressIndicator());
          }
          if (state.errorMessage != null && state.errorMessage!.isNotEmpty) {
            return Center(child: Text('搜索失败: ${state.errorMessage}'));
          }
          if (state.results.isEmpty) {
            return const Center(child: Text('暂无搜索结果'));
          }
          return GridView.builder(
            padding: const EdgeInsets.all(16),
            itemCount: state.results.length,
            gridDelegate: gridDelegate,
            itemBuilder: (context, index) {
              final book = state.results[index];
              return BookCard(
                book: book,
                onTap: () {
                  Navigator.push(
                    context,
                    MaterialPageRoute(
                      builder: (context) => BookstoreDetailPage(book: book),
                    ),
                  );
                },
              );
            },
          );
        },
      ),
    );
  }
}
