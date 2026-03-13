import 'package:flutter/material.dart';

/// 搜索栏组件
/// 支持搜索按钮和搜索框两种状态的切换
class CustomSearchBar extends StatefulWidget {
  final Function(String) onSearch; // 搜索回调函数
  final Function() onCancel; // 取消搜索回调函数

  const CustomSearchBar({
    Key? key,
    required this.onSearch,
    required this.onCancel,
  }) : super(key: key);

  @override
  State<CustomSearchBar> createState() => _CustomSearchBarState();
}

/// 搜索栏状态类
class _CustomSearchBarState extends State<CustomSearchBar> {
  final TextEditingController _searchController = TextEditingController(); // 搜索文本控制器
  bool _isSearching = false; // 是否处于搜索状态

  @override
  Widget build(BuildContext context) {
    if (!_isSearching) {
      // 搜索按钮状态
      return IconButton(
        icon: const Icon(Icons.search), // 搜索图标
        onPressed: () {
          // 切换到搜索框状态
          setState(() {
            _isSearching = true;
          });
        },
      );
    } else {
      // 搜索框状态
      return Container(
        width: 200, // 搜索框宽度
        height: 40, // 搜索框高度
        decoration: BoxDecoration(
          color: Colors.grey[200], // 搜索框背景色
          borderRadius: BorderRadius.circular(20), // 圆角
        ),
        child: Row(
          children: [
            const SizedBox(width: 10), // 左侧间距
            const Icon(
              Icons.search,
              color: Colors.grey,
              size: 20, // 搜索图标
            ),
            const SizedBox(width: 6), // 图标与输入框间距
            Expanded(
              child: TextField(
                controller: _searchController, // 文本控制器
                textAlignVertical: TextAlignVertical.center,
                decoration: const InputDecoration(
                  border: InputBorder.none, // 无边框
                  hintText: '搜索书籍', // 提示文本
                  hintStyle: TextStyle(
                    color: Colors.grey,
                    fontSize: 14,
                  ),
                  isDense: true,
                  contentPadding: EdgeInsets.symmetric(vertical: 8),
                ),
                onSubmitted: (value) {
                  // 提交搜索
                  widget.onSearch(value);
                },
              ),
            ),
            // 只有当搜索框中有文本时才显示清除按钮
            if (_searchController.text.isNotEmpty)
              IconButton(
                icon: const Icon(Icons.clear, size: 18), // 清除图标
                onPressed: () {
                  _searchController.clear(); // 清除输入
                  setState(() {});
                },
              ),
            // 关闭搜索按钮
            IconButton(
              icon: const Icon(Icons.close, size: 18), // 关闭图标
              onPressed: () {
                // 切换回搜索按钮状态
                setState(() {
                  _isSearching = false;
                  _searchController.clear(); // 清除输入
                  widget.onCancel(); // 调用取消回调
                });
              },
            ),
          ],
        ),
      );
    }
  }
}