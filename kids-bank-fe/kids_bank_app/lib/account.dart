import 'package:flutter/material.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
import 'app_config.dart';
import 'string_exts.dart';

class User {
  final String name;
  final String email;
  final String password;

  User({required this.name, required this.email, required this.password});

  factory User.fromJson(Map<String, dynamic> json) {
    return User(
      name: json["name"],
      email: json["email"],
      password: json["password"]
    );
  }
}

class Account {
  final String id;
  final User user;
  double balance;
  final double currentApr;

  Account({required this.id, required this.user, required this.balance, required this.currentApr});

  factory Account.fromJson(Map<String, dynamic> json) {
    return Account(
      id: json["id"],
      user: User.fromJson(json["user"]),
      currentApr: json["current_apr"],
      balance: (json["balance"] as num).toDouble(),
    );
  }
}

class AccountScreen extends StatefulWidget {
  final Account account;

  const AccountScreen({super.key, required this.account});

  @override
  AccountScreenState createState() => AccountScreenState();
}

class AccountScreenState extends State<AccountScreen> {
  final String _baseUrl = AppConfig.BASE_API_URL;
  final FlutterSecureStorage _storage = const FlutterSecureStorage(
    aOptions: AndroidOptions(encryptedSharedPreferences: true),
    iOptions: IOSOptions(accessibility: KeychainAccessibility.first_unlock),
    lOptions: LinuxOptions(),
  );

  void _showSnackbar(String message) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(message),
        backgroundColor: Colors.red,
        behavior: SnackBarBehavior.floating,
      ),
    );
  }

  Future<void> _adjustAccount(String endpoint, String id, double amount) async {
    final token = await _storage.read(key: "token");
    final response = await http.post(
      Uri.parse("$_baseUrl/$endpoint?id=$id&amount=$amount"),
      headers: {"Authorization": "Bearer $token"},
    );

    if (response.statusCode > 399) {
      _showSnackbar("Failed to $endpoint the amount. Please try again.");
    } else {
      setState(() {
        widget.account.balance = jsonDecode(response.body)["balance"].toDouble();
      });
    }
  }

  void _showAmountDialog(String buttonText, String action) {
    final TextEditingController amountController = TextEditingController();
    showDialog(
      context: context,
      builder: (context) {
        return AlertDialog(
          title: Text("$buttonText Funds"),
          content: TextField(
            controller: amountController,
            keyboardType: TextInputType.number,
            decoration: InputDecoration(
              labelText: "Enter amount",
              border: OutlineInputBorder(),
            ),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context),
              child: Text("Cancel"),
            ),
            ElevatedButton(
              onPressed: () {
                final amount = double.tryParse(amountController.text);
                if (amount != null && amount > 0) {
                  _adjustAccount(action.toLowerCase(), widget.account.id, amount);
                  Navigator.pop(context);
                }
              },
              child: Text("$buttonText Funds"),
            ),
          ],
        );
      },
    );
  }
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("${widget.account.user.name.capitalize()}'s Account Details"),
        backgroundColor: Colors.blue,
        foregroundColor: Colors.white,
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Container(
              padding: EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Colors.blue,
                borderRadius: BorderRadius.circular(8),
              ),
              child: Text(
                "Welcome, ${widget.account.user.name.capitalize()}!",
                style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold, color: Colors.white),
              ),
            ),
            const SizedBox(height: 20),
            Text("Balance: \$${widget.account.balance.toStringAsFixed(2)}", style: TextStyle(fontSize: 24)),
            const SizedBox(height: 20),
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                ElevatedButton(
                  onPressed: () => _showAmountDialog("Add", "deposit"),
                  child: Text("Add Funds"),
                ),
                const SizedBox(width: 10),
                ElevatedButton(
                  onPressed: widget.account.balance > 0 ? () => _showAmountDialog("Withdraw", "withdraw") : null,
                  child: Text("Withdraw Funds"),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
