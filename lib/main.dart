import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:textrest/src/rust/frb_generated.dart';
import 'package:flutter/services.dart';
import 'package:textrest/src/app/router/app_router.dart';
import 'package:textrest/src/rust/api/data_base.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await RustLib.init(); // 初始化 Rust 库
  try {
    await initDatabase(); // 初始化数据库
  } catch (_) {
    // 已初始化时忽略
  }
  runApp(const ProviderScope(child: TextRestApp()));
  // 设置 Android 状态栏透明
  if(Platform.isAndroid){
    SystemUiOverlayStyle systemUiOverlayStyle =
        SystemUiOverlayStyle(statusBarColor: Colors.transparent);
    SystemChrome.setSystemUIOverlayStyle(systemUiOverlayStyle);
  }
}


class TextRestApp extends StatelessWidget {
  const TextRestApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      title: 'TextRest',
      // 去除Dbug
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: const Color(0xFF4F46E5)),
        useMaterial3: true,
      ),
      routerConfig: appRouter,
    );
  }
}
