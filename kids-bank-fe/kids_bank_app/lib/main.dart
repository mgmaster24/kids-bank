import 'package:flutter/material.dart';
import 'package:kids_bank_app/login.dart';

void main() {
  runApp(const KidsBankApp());
}

class KidsBankApp extends StatelessWidget {
  const KidsBankApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Kids Bank',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const LoginScreen(),
    );
  }
}

