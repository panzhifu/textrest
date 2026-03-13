import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:textrest/src/app/features/setting/book_source_edit_page.dart';
import 'package:textrest/src/app/features/setting/provider/book_source_controller.dart';
import 'package:textrest/src/rust/models/book_source.dart';

class BookSourceImportPage extends ConsumerStatefulWidget {
  const BookSourceImportPage({super.key});

  @override
  ConsumerState<BookSourceImportPage> createState() => _BookSourceImportPageState();
}

class _BookSourceImportPageState extends ConsumerState<BookSourceImportPage> {
  final TextEditingController _urlController = TextEditingController();
  final TextEditingController _textController = TextEditingController();
  String? _pickedFileName;
  String? _pickedFilePath;

  @override
  void dispose() {
    _urlController.dispose();
    _textController.dispose();
    super.dispose();
  }

  Future<void> _pickFile() async {
    final result = await FilePicker.platform.pickFiles(
      type: FileType.custom,
      allowedExtensions: const ['json', 'txt'],
    );
    if (result == null) return;
    final file = result.files.first;
    setState(() {
      _pickedFileName = file.name;
      _pickedFilePath = file.path;
    });
  }

  Future<void> _importSources() async {
    final controller = ref.read(bookSourceProvider.notifier);
    final url = _urlController.text.trim();
    final text = _textController.text.trim();

    try {
      if (_pickedFilePath != null && _pickedFilePath!.isNotEmpty) {
        await controller.importFromFile(_pickedFilePath!);
      }
      if (url.isNotEmpty) {
        await controller.importFromUrl(url);
      }
      if (text.isNotEmpty) {
        await controller.importFromText(text);
      }
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('导入成功')),
      );
      Navigator.pop(context);
    } catch (error) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('导入失败: $error')),
      );
    }
  }

  void _openEditor() {
    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => BookSourceEditPage(source: _emptySource()),
      ),
    );
  }

  BookSource _emptySource() {
    return const BookSource(
      bookSourceUrl: '',
      bookSourceName: '',
      bookSourceType: 0,
      customOrder: 0,
      enabled: true,
      enabledExplore: false,
      enabledCookieJar: false,
      lastUpdateTime: '',
      respondTime: 0,
      weight: 0,
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('导入书源'),
      ),
      body: SafeArea(
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text(
                '支持链接、文件或粘贴 JSON 内容导入书源',
                style: TextStyle(fontSize: 14, color: Colors.grey),
              ),
              const SizedBox(height: 16),
              TextField(
                controller: _urlController,
                decoration: InputDecoration(
                  labelText: '从 URL 导入',
                  hintText: 'https://example.com/booksource.json',
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(12),
                  ),
                  prefixIcon: const Icon(Icons.link),
                ),
              ),
              const SizedBox(height: 16),
              OutlinedButton.icon(
                onPressed: _pickFile,
                icon: const Icon(Icons.attach_file),
                label: Text(_pickedFileName == null ? '从文件导入' : '已选择：$_pickedFileName'),
              ),
              const SizedBox(height: 16),
              TextField(
                controller: _textController,
                maxLines: 6,
                decoration: InputDecoration(
                  labelText: '粘贴书源 JSON',
                  hintText: '请输入书源 JSON 内容',
                  alignLabelWithHint: true,
                  border: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(12),
                  ),
                ),
              ),
              const Spacer(),
              Row(
                children: [
                  Expanded(
                    child: OutlinedButton.icon(
                      onPressed: _openEditor,
                      icon: const Icon(Icons.edit_outlined),
                      label: const Text('编辑书源'),
                    ),
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: ElevatedButton.icon(
                      onPressed: _importSources,
                      icon: const Icon(Icons.file_download),
                      label: const Text('导入书源'),
                    ),
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }
}
