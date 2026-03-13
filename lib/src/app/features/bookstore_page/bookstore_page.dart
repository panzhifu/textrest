import 'package:flutter/material.dart';
import 'package:textrest/src/app/features/bookstore_page/bookstore_search_page.dart';
import 'package:textrest/src/app/widgets/search_bar.dart';

class BookstorePage extends StatelessWidget {
  const BookstorePage({super.key});

  void _openSearch(BuildContext context, String keyword) {
    if (keyword.trim().isEmpty) return;
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => BookstoreSearchPage(keyword: keyword),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('书城'),
        actions: [
          CustomSearchBar(
            onSearch: (value) => _openSearch(context, value),
            onCancel: () {},
          ),
        ],
      ),
      body: const Center(
        child: Text('书城页面'),
      ),
    );
  }
}
