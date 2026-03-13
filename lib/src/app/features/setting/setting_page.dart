import 'package:flutter/material.dart';
import 'package:settings_ui/settings_ui.dart';
import 'package:textrest/src/app/features/setting/book_source_page.dart';
import 'package:textrest/src/app/features/setting/about.dart';

class SettingPage extends StatelessWidget {
  const SettingPage({super.key});

  @override
  Widget build(BuildContext context) {
    final dividerColor = Theme.of(context).dividerColor.withOpacity(0.15);
    final cardColor = Theme.of(context).colorScheme.surface;

    return Scaffold(
      appBar: AppBar(title: const Text('设置')),
      body: SettingsList(
        sections: [
          CustomSettingsSection(
            child: _SettingsCard(
              title: '书源',
              cardColor: cardColor,
              dividerColor: dividerColor,
              children: [
                _SettingsTile(
                  icon: Icons.menu_book,
                  title: '书源管理',
                  subtitle: '新建 · 导入 · 编辑或管理书源',
                  onTap: () {
                    Navigator.push(
                      context,
                      MaterialPageRoute(
                        builder: (context) => const BookSourcePage(),
                      ),
                    );
                  },
                ),
              ],
            ),
          ),
          CustomSettingsSection(
            child: _SettingsCard(
              title: '阅读',
              cardColor: cardColor,
              dividerColor: dividerColor,
              children: const [
                _SettingsTile(
                  icon: Icons.text_fields,
                  title: '字体大小',
                  subtitle: '调整阅读字体大小',
                  trailingText: '默认',
                ),
                _SettingsTile(
                  icon: Icons.color_lens_outlined,
                  title: '主题颜色',
                  subtitle: '浅色 / 深色',
                  trailingText: '系统',
                ),
              ],
            ),
          ),
          CustomSettingsSection(
            child: _SettingsCard(
              title: '关于',
              cardColor: cardColor,
              dividerColor: dividerColor,
              children: [
                _SettingsTile(
                  icon: Icons.info_outline,
                  title: '关于',
                  subtitle: '关于应用的版本信息',
                  onTap: () {
                    Navigator.push(
                      context,
                      MaterialPageRoute(builder: (context) => const AboutPage()),
                    );
                  },
                ),
              ],
            ),
          ),
          const CustomSettingsSection(child: SizedBox(height: 24)),
        ],
      ),
    );
  }
}

class _SettingsCard extends StatelessWidget {
  final String title;
  final Color cardColor;
  final Color dividerColor;
  final List<Widget> children;

  const _SettingsCard({
    required this.title,
    required this.cardColor,
    required this.dividerColor,
    required this.children,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 12, 16, 0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            title,
            style: Theme.of(context).textTheme.titleSmall?.copyWith(
                  color: Theme.of(context).colorScheme.primary,
                  fontWeight: FontWeight.w600,
                ),
          ),
          const SizedBox(height: 10),
          Container(
            decoration: BoxDecoration(
              color: cardColor,
              borderRadius: BorderRadius.circular(18),
            ),
            child: Column(
              children: _withDividers(children, dividerColor),
            ),
          ),
        ],
      ),
    );
  }

  List<Widget> _withDividers(List<Widget> tiles, Color dividerColor) {
    final items = <Widget>[];
    for (var i = 0; i < tiles.length; i++) {
      items.add(tiles[i]);
      if (i != tiles.length - 1) {
        items.add(Divider(height: 1, color: dividerColor));
      }
    }
    return items;
  }
}

class _SettingsTile extends StatelessWidget {
  final IconData icon;
  final String title;
  final String subtitle;
  final String? trailingText;
  final VoidCallback? onTap;

  const _SettingsTile({
    required this.icon,
    required this.title,
    required this.subtitle,
    this.trailingText,
    this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final iconColor = Theme.of(context).colorScheme.onSurfaceVariant;
    final subtitleColor = Theme.of(context).colorScheme.onSurfaceVariant;
    return ListTile(
      leading: Icon(icon, color: iconColor),
      title: Text(title),
      subtitle: Text(subtitle, style: TextStyle(color: subtitleColor)),
      trailing: trailingText != null
          ? Text(
              trailingText!,
              style: TextStyle(color: subtitleColor),
            )
          : const Icon(Icons.chevron_right),
      onTap: onTap,
    );
  }
}
