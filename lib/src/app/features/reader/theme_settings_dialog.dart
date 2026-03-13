import 'package:flutter/material.dart';

enum ReadingTheme {
  light,
  sepia,
  dark,
}

class ThemeSettingsDialog extends StatefulWidget {
  final ReadingTheme currentTheme;
  final double fontSize;
  final double lineHeight;
  final Function(ReadingTheme) onThemeChanged;
  final Function(double) onFontSizeChanged;
  final Function(double) onLineHeightChanged;

  const ThemeSettingsDialog({
    super.key,
    required this.currentTheme,
    required this.fontSize,
    required this.lineHeight,
    required this.onThemeChanged,
    required this.onFontSizeChanged,
    required this.onLineHeightChanged,
  });

  @override
  State<ThemeSettingsDialog> createState() => _ThemeSettingsDialogState();
}

class _ThemeSettingsDialogState extends State<ThemeSettingsDialog> {
  late ReadingTheme _selectedTheme;
  late double _fontSize;
  late double _lineHeight;

  @override
  void initState() {
    super.initState();
    _selectedTheme = widget.currentTheme;
    _fontSize = widget.fontSize;
    _lineHeight = widget.lineHeight;
  }

  @override
  Widget build(BuildContext context) {
    return Dialog(
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(16),
      ),
      child: Container(
        constraints: const BoxConstraints(maxWidth: 400),
        padding: const EdgeInsets.all(16),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                const Icon(Icons.palette, size: 24),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    '阅读设置',
                    style: Theme.of(context).textTheme.titleLarge,
                  ),
                ),
                IconButton(
                  icon: const Icon(Icons.close),
                  onPressed: () => Navigator.pop(context),
                ),
              ],
            ),
            const SizedBox(height: 16),
            Text(
              '主题',
              style: Theme.of(context).textTheme.titleMedium,
            ),
            const SizedBox(height: 8),
            Row(
              children: [
                _ThemeOption(
                  theme: ReadingTheme.light,
                  selectedTheme: _selectedTheme,
                  label: '浅色',
                  backgroundColor: Colors.white,
                  textColor: Colors.black87,
                  onTap: () => setState(() => _selectedTheme = ReadingTheme.light),
                ),
                const SizedBox(width: 8),
                _ThemeOption(
                  theme: ReadingTheme.sepia,
                  selectedTheme: _selectedTheme,
                  label: '护眼',
                  backgroundColor: const Color(0xFFF5F0E1),
                  textColor: Colors.brown,
                  onTap: () => setState(() => _selectedTheme = ReadingTheme.sepia),
                ),
                const SizedBox(width: 8),
                _ThemeOption(
                  theme: ReadingTheme.dark,
                  selectedTheme: _selectedTheme,
                  label: '深色',
                  backgroundColor: Colors.grey[900]!,
                  textColor: Colors.white,
                  onTap: () => setState(() => _selectedTheme = ReadingTheme.dark),
                ),
              ],
            ),
            const SizedBox(height: 16),
            Text(
              '字体大小: ${_fontSize.toStringAsFixed(0)}',
              style: Theme.of(context).textTheme.titleMedium,
            ),
            Slider(
              value: _fontSize,
              min: 12,
              max: 28,
              divisions: 16,
              onChanged: (value) => setState(() => _fontSize = value),
            ),
            const SizedBox(height: 8),
            Text(
              '行间距: ${_lineHeight.toStringAsFixed(1)}',
              style: Theme.of(context).textTheme.titleMedium,
            ),
            Slider(
              value: _lineHeight,
              min: 1.0,
              max: 2.5,
              divisions: 15,
              onChanged: (value) => setState(() => _lineHeight = value),
            ),
            const SizedBox(height: 24),
            Row(
              children: [
                Expanded(
                  child: OutlinedButton(
                    onPressed: () => Navigator.pop(context),
                    child: const Text('取消'),
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: ElevatedButton(
                    onPressed: () {
                      widget.onThemeChanged(_selectedTheme);
                      widget.onFontSizeChanged(_fontSize);
                      widget.onLineHeightChanged(_lineHeight);
                      Navigator.pop(context);
                    },
                    child: const Text('确定'),
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

class _ThemeOption extends StatelessWidget {
  final ReadingTheme theme;
  final ReadingTheme selectedTheme;
  final String label;
  final Color backgroundColor;
  final Color textColor;
  final VoidCallback onTap;

  const _ThemeOption({
    required this.theme,
    required this.selectedTheme,
    required this.label,
    required this.backgroundColor,
    required this.textColor,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final isSelected = theme == selectedTheme;
    return Expanded(
      child: GestureDetector(
        onTap: onTap,
        child: Container(
          padding: const EdgeInsets.symmetric(vertical: 16, horizontal: 8),
          decoration: BoxDecoration(
            color: backgroundColor,
            borderRadius: BorderRadius.circular(8),
            border: Border.all(
              color: isSelected ? Colors.blue : Colors.grey[300]!,
              width: isSelected ? 2 : 1,
            ),
          ),
          child: Column(
            children: [
              Text(
                'Aa',
                style: TextStyle(
                  color: textColor,
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                ),
              ),
              const SizedBox(height: 4),
              Text(
                label,
                style: TextStyle(
                  color: textColor,
                  fontSize: 12,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
