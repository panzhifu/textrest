import 'package:flutter/material.dart';

class ConverterPage extends StatelessWidget {
  const ConverterPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('格式转换'),
      ),
      body: const Center(
        child: Text('格式转换页面'),
      ),
    );
  }
}