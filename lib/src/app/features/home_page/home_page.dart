import 'package:flutter/material.dart';

import 'package:textrest/src/app/widgets/app_nav.dart';
import 'package:textrest/src/app/features/bookshelf_page/bookshelf_page.dart';
import 'package:textrest/src/app/features/bookstore_page/bookstore_page.dart';
import 'package:textrest/src/app/features/statistics_page/statistics_page.dart';
import 'package:textrest/src/app/features/converter_page/converter_page.dart';
import 'package:textrest/src/app/features/setting/setting_page.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  int _currentIndex = 0;

  late final List<NavItem> _items = [
    const NavItem(
      icon: Icons.book,
      title: '书架',
      page: BookshelfPage(),
    ),
    const NavItem(
      icon: Icons.store,
      title: '书城',
      page: BookstorePage(),
    ),
    const NavItem(
      icon: Icons.bar_chart,
      title: '统计',
      page: StatisticsPage(),
    ),
    const NavItem(
      icon: Icons.transform,
      title: '转换',
      page: ConverterPage(),
    ),
    const NavItem(
      icon: Icons.settings,
      title: '设置',
      page: SettingPage(),
    ),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: AppNav(
        currentIndex: _currentIndex,
        items: _items,
        onTap: (index) {
          setState(() {
            _currentIndex = index;
          });
        },
      ),
    );
  }
}