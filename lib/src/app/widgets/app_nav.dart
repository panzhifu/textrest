import 'package:flutter/material.dart';

/// 应用导航组件
class AppNav extends StatefulWidget {
  final int currentIndex;
  final ValueChanged<int> onTap;
  final List<NavItem> items;

  const AppNav({
    super.key,
    required this.currentIndex,
    required this.onTap,
    required this.items,
  });

  @override
  State<AppNav> createState() => _AppNavState();
}

class _AppNavState extends State<AppNav> {
  @override
  Widget build(BuildContext context) {
    final isMobile = MediaQuery.of(context).size.width < 600;

    if (isMobile) {
      return Stack(
        children: [
          widget.items[widget.currentIndex].page,
          Positioned(
            left: 16,
            right: 16,
            bottom: 16,
            child: SafeArea(
              top: false,
              child: Material(
                elevation: 6,
                color: Theme.of(context).colorScheme.surface,
                shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(28)),
                child: ClipRRect(
                  borderRadius: BorderRadius.circular(28),
                  child: NavigationBarTheme(
                    data: NavigationBarThemeData(
                      height: 64,
                      backgroundColor: Theme.of(context).colorScheme.surface,
                      indicatorColor: Colors.transparent,
                      labelBehavior: NavigationDestinationLabelBehavior.alwaysShow,
                      labelTextStyle: WidgetStateProperty.resolveWith(
                        (states) => TextStyle(
                          fontSize: 12,
                          fontWeight: FontWeight.w600,
                          color: states.contains(WidgetState.selected)
                              ? Theme.of(context).colorScheme.primary
                              : Theme.of(context).colorScheme.onSurfaceVariant,
                        ),
                      ),
                    ),
                    child: NavigationBar(
                      selectedIndex: widget.currentIndex,
                      onDestinationSelected: widget.onTap,
                      destinations: widget.items
                          .map(
                            (item) => NavigationDestination(
                              icon: Icon(
                                item.icon,
                                size: 22,
                                color: Theme.of(context).colorScheme.onSurfaceVariant,
                              ),
                              selectedIcon: Icon(
                                item.icon,
                                size: 22,
                                color: Theme.of(context).colorScheme.primary,
                              ),
                              label: item.title,
                            ),
                          )
                          .toList(),
                    ),
                  ),
                ),
              ),
            ),
          ),
        ],
      );
    }

    const railWidth = 80.0;
    const railMargin = 20.0;

    return Stack(
      children: [
        Padding(
          padding: const EdgeInsets.only(left: railWidth + railMargin * 2),
          child: widget.items[widget.currentIndex].page,
        ),
        Positioned(
          left: railMargin,
          top: railMargin,
          bottom: railMargin,
          child: SafeArea(
            right: false,
            child: Material(
              elevation: 6,
              color: Theme.of(context).colorScheme.surface,
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(28)),
              child: SizedBox(
                width: railWidth,
                child: ClipRRect(
                  borderRadius: BorderRadius.circular(28),
                  child: NavigationRail(
                    groupAlignment: 1.0,
                    destinations: widget.items
                        .map(
                          (item) => NavigationRailDestination(
                            icon: Icon(item.icon),
                            label: Text(item.title),
                            padding: const EdgeInsets.symmetric(vertical: 18),
                          ),
                        )
                        .toList(),
                    selectedIndex: widget.currentIndex,
                    onDestinationSelected: widget.onTap,
                    extended: false,
                  ),
                ),
              ),
            ),
          ),
        ),
      ],
    );
  }
}

class NavItem {
  final IconData icon;
  final String title;
  final Widget page;

  const NavItem({
    required this.icon,
    required this.title,
    required this.page,
  });
}

