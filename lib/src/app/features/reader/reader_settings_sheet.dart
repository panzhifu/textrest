import 'package:flutter/material.dart';

class ReaderSettingsSheet extends StatefulWidget {
  final double fontSize;
  final double lineHeight;
  final ValueChanged<double> onFontSizeChanged;
  final ValueChanged<double> onLineHeightChanged;

  const ReaderSettingsSheet({
    super.key,
    required this.fontSize,
    required this.lineHeight,
    required this.onFontSizeChanged,
    required this.onLineHeightChanged,
  });

  @override
  State<ReaderSettingsSheet> createState() => _ReaderSettingsSheetState();
}

class _ReaderSettingsSheetState extends State<ReaderSettingsSheet> {
  late double _fontSize;
  late double _lineHeight;

  @override
  void initState() {
    super.initState();
    _fontSize = widget.fontSize;
    _lineHeight = widget.lineHeight;
  }

  @override
  Widget build(BuildContext context) {
    final surface = Theme.of(context).colorScheme.surface;
    final onSurface = Theme.of(context).colorScheme.onSurfaceVariant;
    final primary = Theme.of(context).colorScheme.primary;

    return SafeArea(
      top: false,
      child: Container(
        margin: const EdgeInsets.fromLTRB(16, 0, 16, 16),
        child: Material(
          elevation: 6,
          color: surface,
          shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(28)),
          child: Padding(
            padding: const EdgeInsets.fromLTRB(16, 12, 16, 16),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                Container(
                  width: 40,
                  height: 4,
                  decoration: BoxDecoration(
                    color: onSurface.withOpacity(0.4),
                    borderRadius: BorderRadius.circular(2),
                  ),
                ),
                const SizedBox(height: 12),
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceAround,
                  children: [
                    _TopAction(icon: Icons.menu_book, label: '目录'),
                    _TopAction(icon: Icons.tune, label: '设置', isActive: true),
                    _TopAction(icon: Icons.brightness_6_outlined, label: '亮度'),
                    _TopAction(icon: Icons.color_lens_outlined, label: '主题'),
                  ],
                ),
                const SizedBox(height: 12),
                Divider(height: 1, color: onSurface.withOpacity(0.2)),
                const SizedBox(height: 12),
                _SettingRow(
                  title: '字体大小',
                  value: _fontSize.toStringAsFixed(0),
                  accentColor: primary,
                  child: Slider(
                    value: _fontSize,
                    min: 12,
                    max: 28,
                    onChanged: (value) {
                      setState(() => _fontSize = value);
                      widget.onFontSizeChanged(value);
                    },
                  ),
                ),
                const SizedBox(height: 8),
                _SettingRow(
                  title: '行间距',
                  value: _lineHeight.toStringAsFixed(1),
                  accentColor: primary,
                  child: Slider(
                    value: _lineHeight,
                    min: 1.2,
                    max: 2.2,
                    onChanged: (value) {
                      setState(() => _lineHeight = value);
                      widget.onLineHeightChanged(value);
                    },
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

class _TopAction extends StatelessWidget {
  final IconData icon;
  final String label;
  final bool isActive;

  const _TopAction({
    required this.icon,
    required this.label,
    this.isActive = false,
  });

  @override
  Widget build(BuildContext context) {
    final color = isActive
        ? Theme.of(context).colorScheme.primary
        : Theme.of(context).colorScheme.onSurfaceVariant;

    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Icon(icon, size: 22, color: color),
        const SizedBox(height: 4),
        Text(
          label,
          style: Theme.of(context).textTheme.labelSmall?.copyWith(
                color: color,
                fontWeight: FontWeight.w600,
              ),
        ),
      ],
    );
  }
}

class _SettingRow extends StatelessWidget {
  final String title;
  final String value;
  final Widget child;
  final Color accentColor;

  const _SettingRow({
    required this.title,
    required this.value,
    required this.child,
    required this.accentColor,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          children: [
            Text(
              title,
              style: Theme.of(context).textTheme.titleSmall?.copyWith(
                    fontWeight: FontWeight.w600,
                  ),
            ),
            const Spacer(),
            Text(
              value,
              style: Theme.of(context).textTheme.labelLarge?.copyWith(
                    color: accentColor,
                  ),
            ),
          ],
        ),
        child,
      ],
    );
  }
}
