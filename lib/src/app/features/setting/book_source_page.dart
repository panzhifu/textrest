import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:textrest/src/app/features/setting/provider/book_source_controller.dart';
import 'package:textrest/src/app/features/setting/book_source_edit_page.dart';
import 'package:textrest/src/app/features/setting/book_source_import_page.dart';
import 'package:textrest/src/app/widgets/search_bar.dart';

class BookSourcePage extends ConsumerWidget {
  const BookSourcePage({super.key});

  void _showManageSheet(BuildContext context, WidgetRef ref) {
    final state = ref.read(bookSourceProvider);
    final controller = ref.read(bookSourceProvider.notifier);
    final allSelected = state.selected.length == state.filtered.length && state.filtered.isNotEmpty;

    showModalBottomSheet<void>(
      context: context,
      backgroundColor: Colors.transparent,
      builder: (context) {
        return SafeArea(
          top: false,
          child: Container(
            margin: const EdgeInsets.fromLTRB(16, 0, 16, 16),
            child: Material(
              elevation: 6,
              color: Theme.of(context).colorScheme.surface,
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(28)),
              child: Padding(
                padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                child: Row(
                  children: [
                    Row(
                      children: [
                        Checkbox(
                          value: allSelected,
                          onChanged: (value) => controller.toggleSelectAll(value ?? false),
                        ),
                        const Text('全选'),
                      ],
                    ),
                    const Spacer(),
                    TextButton.icon(
                      onPressed:
                          state.selected.isEmpty ? null : () async => controller.toggleEnabledForSelected(true),
                      icon: const Icon(Icons.check_circle_outline),
                      label: const Text('启用'),
                    ),
                    const SizedBox(width: 8),
                    TextButton.icon(
                      onPressed:
                          state.selected.isEmpty ? null : () async => controller.toggleEnabledForSelected(false),
                      icon: const Icon(Icons.block),
                      label: const Text('停用'),
                    ),
                    const SizedBox(width: 8),
                    TextButton.icon(
                      onPressed: state.selected.isEmpty ? null : () async => controller.deleteSelected(),
                      icon: const Icon(Icons.delete_outline),
                      label: const Text('删除'),
                    ),
                  ],
                ),
              ),
            ),
          ),
        );
      },
    );
  }

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(bookSourceProvider);
    final controller = ref.read(bookSourceProvider.notifier);
    final sources = state.filtered;

    return Scaffold(
      appBar: AppBar(
        title: const Text('书源管理'),
        actions: [
          CustomSearchBar(
            onSearch: controller.updateKeyword,
            onCancel: controller.clearKeyword,
          ),
          IconButton(
            icon: const Icon(Icons.add),
            onPressed: () {
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (context) => const BookSourceImportPage(),
                ),
              );
            },
          ),
        ],
      ),
      body: state.isLoading
          ? const Center(child: CircularProgressIndicator())
          : state.error != null
              ? Center(child: Text('加载失败: ${state.error}'))
              : sources.isEmpty
                  ? const Center(child: Text('暂无书源'))
                  : ListView.separated(
                      itemCount: sources.length,
                      separatorBuilder: (_, __) => const Divider(height: 1),
                      itemBuilder: (context, index) {
                        final source = sources[index];
                        final isSelected = state.selected.contains(index);
                        return ListTile(
                          leading: Checkbox(
                            value: isSelected,
                            onChanged: (_) => controller.toggleSelect(index),
                          ),
                          title: Text(source.bookSourceName),
                          subtitle: Text(source.enabled ? '状态：启用' : '状态：停用'),
                          trailing: Row(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              Switch(
                                value: source.enabled,
                                onChanged: (value) => controller.toggleSingle(source, value),
                              ),
                              IconButton(
                                icon: const Icon(Icons.more_horiz),
                                onPressed: () {
                                  controller.toggleSelect(index);
                                  _showManageSheet(context, ref);
                                },
                              ),
                            ],
                          ),
                          onTap: () {
                            Navigator.push(
                              context,
                              MaterialPageRoute(
                                builder: (context) => BookSourceEditPage(
                                  source: source,
                                ),
                              ),
                            );
                          },
                        );
                      },
                    ),
    );
  }
}
