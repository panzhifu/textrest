import 'package:go_router/go_router.dart';

import 'package:textrest/src/app/features/home_page/home_page.dart';
import 'package:textrest/src/app/features/reader/reader.dart';

// 使用 go_router 配置路由
final appRouter = GoRouter(
  initialLocation: '/',
  routes: [
    GoRoute(
      path: '/',
      builder: (context, state) => const HomePage(),
    ),
    GoRoute(
      path: '/reader',
      builder: (context, state) {
        final bookId = state.extra as String?;
        return ReaderPage(bookId: bookId ?? '');
      },
    ),
  ],
);

