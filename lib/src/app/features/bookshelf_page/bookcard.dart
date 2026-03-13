import 'dart:io';
import 'package:flutter/material.dart';
import 'package:textrest/src/rust/models/book.dart';

/// 书籍卡片组件
/// 用于在书架界面展示书籍信息
class BookCard extends StatelessWidget {
  final Book book; // 书籍对象
  final VoidCallback? onTap; // 点击回调
  final VoidCallback? onLongPress; // 长按回调

  const BookCard({
    super.key,
    required this.book,
    this.onTap,
    this.onLongPress,
  });

  /// 封面图片解析，支持本地文件和网络图片
  ImageProvider? _resolveCoverImage() {
    final coverUrl = book.coverUrl?.trim();
    if (coverUrl == null || coverUrl.isEmpty) return null;

    if (coverUrl.startsWith('http://') || coverUrl.startsWith('https://')) {
      return NetworkImage(coverUrl);
    }

    if (coverUrl.startsWith('/') || coverUrl.contains(':')) {
      final file = File(coverUrl);
      if (file.existsSync()) {
        return FileImage(file);
      }
    }

    return null;
  }

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: onTap, // 点击事件
      onLongPress: onLongPress,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start, // 左对齐
        mainAxisSize: MainAxisSize.max,
        children: [
          // 书籍封面
          Expanded(
            child: AspectRatio(
              aspectRatio: 0.7,
              child: Container(
                width: double.infinity,
                decoration: BoxDecoration(
                  borderRadius: BorderRadius.circular(8.0), // 圆角
                  color: Colors.grey[200], // 默认背景色
                  boxShadow: [
                    BoxShadow(
                      color: Colors.black.withOpacity(0.1),
                      blurRadius: 4,
                      offset: const Offset(0, 2),
                    ),
                  ],
                ),
                child: _resolveCoverImage() == null
                    ? Center(
                        child: Text(
                          book.name.isNotEmpty ? book.name.substring(0, 1) : '?', // 显示书名首字母
                          style: const TextStyle(
                            fontSize: 40,
                            color: Colors.grey,
                          ),
                        ),
                      )
                    : ClipRRect(
                        borderRadius: BorderRadius.circular(8.0),
                        child: Image(
                          image: _resolveCoverImage()!,
                          fit: BoxFit.cover,
                          errorBuilder: (context, error, stackTrace) {
                            return Center(
                              child: Text(
                                book.name.isNotEmpty
                                    ? book.name.substring(0, 1)
                                    : '?',
                                style: const TextStyle(
                                  fontSize: 40,
                                  color: Colors.grey,
                                ),
                              ),
                            );
                          },
                        ),
                      ),
              ),
            ),
          ),
          // 书籍标题
          const SizedBox(height: 8.0), // 间距
          Text(
            book.name, // 书名
            maxLines: 2, // 最多显示2行
            overflow: TextOverflow.ellipsis, // 溢出显示省略号
            style: const TextStyle(
              fontSize: 14,
              fontWeight: FontWeight.bold,
            ),
          ),
          // 作者
          const SizedBox(height: 2.0),
          Text(
            book.author, // 作者名
            maxLines: 1, // 最多显示1行
            overflow: TextOverflow.ellipsis, // 溢出显示省略号
            style: const TextStyle(
              fontSize: 12,
              color: Colors.grey,
            ),
          ),
        ],
      ),
    );
  }
}
