import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:textrest/src/app/features/bookshelf_page/bookcard.dart';
import 'package:textrest/src/app/features/reader/reader.dart';
import 'package:textrest/src/app/widgets/search_bar.dart';
import 'package:textrest/src/rust/api/book.dart';
import 'package:textrest/src/rust/models/book.dart';

/// 书架页面组件
/// 用于展示和管理用户的书籍
class BookshelfPage extends StatefulWidget {
  const BookshelfPage({super.key});

  @override
  State<BookshelfPage> createState() => _BookshelfPageState();
}

class _BookshelfPageState extends State<BookshelfPage> {
  static const List<String> _supportedExtensions = ['epub', 'pdf', 'txt', 'mobi'];

  static const _mobileGridDelegate = SliverGridDelegateWithFixedCrossAxisCount(
    crossAxisCount: 3,
    crossAxisSpacing: 8,
    mainAxisSpacing: 12,
    childAspectRatio: 0.6,
  );

  static const _desktopGridDelegate = SliverGridDelegateWithFixedCrossAxisCount(
    crossAxisCount: 5,
    crossAxisSpacing: 12,
    mainAxisSpacing: 16,
    childAspectRatio: 0.6,
  );

  BookApi? _bookApi;
  List<Book> _books = const [];
  bool _isLoading = true;
  String? _error;

  @override
  void initState() {
    super.initState();
    _initBooks();
  }

  Future<void> _initBooks() async {
    setState(() {
      _isLoading = true;
      _error = null;
    });

    try {
      _bookApi ??= await BookApi.newInstance();
      final books = await _bookApi!.listBooks();
      if (!mounted) return;
      setState(() {
        _books = books;
        _isLoading = false;
      });
    } catch (error) {
      if (!mounted) return;
      setState(() {
        _error = '$error';
        _isLoading = false;
      });
    }
  }

  Future<void> _addBook() async {
    try {
      final result = await FilePicker.platform.pickFiles(
        type: FileType.custom,
        allowedExtensions: _supportedExtensions,
      );

      if (result == null) return;
      final file = result.files.first;
      if (file.path == null) return;

      _bookApi ??= await BookApi.newInstance();
      await _bookApi!.importBook(filePath: file.path!);
      await _initBooks();

      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('添加书籍成功')),
      );
    } catch (error) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('添加书籍失败: $error')),
      );
    }
  }

  Future<void> _deleteBook(Book book) async {
    try {
      _bookApi ??= await BookApi.newInstance();
      await _bookApi!.deleteBook(bookId: book.bookId);
      await _initBooks();

      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('删除书籍成功')),
      );
    } catch (error) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('删除书籍失败: $error')),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    final isMobile = MediaQuery.of(context).size.width < 600;
    final gridDelegate = isMobile ? _mobileGridDelegate : _desktopGridDelegate;

    return Scaffold(
      appBar: AppBar(
        title: const Text('我的书架'),
        actions: [
          CustomSearchBar(
            onSearch: (value) {
              // TODO: 接入搜索逻辑
            },
            onCancel: () {
              // TODO: 取消搜索逻辑
            },
          ),
          IconButton(
            icon: const Icon(Icons.add),
            onPressed: _addBook,
          ),
        ],
      ),
      body: SafeArea(
        child: _isLoading
            ? const Center(child: CircularProgressIndicator())
            : _error != null
                ? Center(child: Text('加载失败: $_error'))
                : _books.isEmpty
                    ? const Center(child: Text('书架为空，点击右上角添加书籍'))
                    : GridView.builder(
                        padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 12),
                        gridDelegate: gridDelegate,
                        itemCount: _books.length,
                        itemBuilder: (context, index) {
                          final book = _books[index];
                          return BookCard(
                            book: book,
                            onTap: () {
                              Navigator.push(
                                context,
                                MaterialPageRoute(
                                  builder: (context) => ReaderPage(bookId: book.bookId),
                                ),
                              );
                            },
                            onLongPress: () {
                              showModalBottomSheet(
                                context: context,
                                builder: (context) {
                                  return SafeArea(
                                    child: Column(
                                      mainAxisSize: MainAxisSize.min,
                                      children: [
                                        ListTile(
                                          leading: const Icon(Icons.delete),
                                          title: const Text('删除书籍'),
                                          onTap: () {
                                            Navigator.pop(context);
                                            showDialog(
                                              context: context,
                                              builder: (context) {
                                                return AlertDialog(
                                                  title: const Text('确认删除'),
                                                  content: Text('确定要删除书籍 "${book.name}" 吗？'),
                                                  actions: [
                                                    TextButton(
                                                      onPressed: () {
                                                        Navigator.pop(context);
                                                      },
                                                      child: const Text('取消'),
                                                    ),
                                                    TextButton(
                                                      onPressed: () async {
                                                        Navigator.pop(context);
                                                        await _deleteBook(book);
                                                      },
                                                      child: const Text('删除'),
                                                    ),
                                                  ],
                                                );
                                              },
                                            );
                                          },
                                        ),
                                      ],
                                    ),
                                  );
                                },
                              );
                            },
                          );
                        },
                      ),
      ),
    );
  }
}
