import 'package:flutter/material.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'dart:convert';
import 'package:http/http.dart' as http;
//import 'package:fluttertoast/fluttertoast.dart';
import 'app_config.dart';
import 'account.dart';

class LoginScreen extends StatefulWidget {
  const LoginScreen({super.key});

  @override
  LoginScreenState createState() => LoginScreenState();
}

class LoginScreenState extends State<LoginScreen> {
  bool isSignUp = false;
  final String _baseUrl = AppConfig.BASE_API_URL;
  final TextEditingController _usernameController = TextEditingController();
  final TextEditingController _emailController = TextEditingController();
  final TextEditingController _passwordController = TextEditingController();
  final TextEditingController _confirmPasswordController = TextEditingController();
  final FlutterSecureStorage _storage = const FlutterSecureStorage(
    aOptions: AndroidOptions(encryptedSharedPreferences: true),
    iOptions: IOSOptions(accessibility: KeychainAccessibility.first_unlock),
    lOptions: LinuxOptions()
  );

  bool hasUppercase = false;
  bool hasNumber = false;
  bool isLongEnough = false;

  bool passwordsMatch = false;
  bool isEmailValid = false;

  void toggleAuthMode() {
    setState(() {
      isSignUp = !isSignUp;
    });
  }

  bool get isFormValid {
    if (isSignUp) {
      return _usernameController.text.isNotEmpty &&
             _emailController.text.isNotEmpty &&
             _passwordController.text.isNotEmpty &&
             _confirmPasswordController.text.isNotEmpty &&
             isLongEnough && hasUppercase && hasNumber && passwordsMatch && isEmailValid;
    }
    return _emailController.text.isNotEmpty && _passwordController.text.isNotEmpty;
  }

  void _validatePassword(String password) {
    if (isSignUp) {
      setState(() {
        hasUppercase = password.contains(RegExp(r'[A-Z]'));
        hasNumber = password.contains(RegExp(r'[0-9]'));
        isLongEnough = password.length >= 8;
        passwordsMatch = _passwordController.text == _confirmPasswordController.text;
      });  
    }

    setState(() {});
  }

  void _validateConfirmPassword(String confirmPassword) {
    setState(() {
      passwordsMatch = _passwordController.text == confirmPassword;
    });
  }

  void _validateEmail(String email) {
    setState(() {
      isEmailValid = RegExp(r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$').hasMatch(email);
    });
  }

  void _showSnackbar(String message) {
  ScaffoldMessenger.of(context).showSnackBar(
    SnackBar(
      content: Text(message),
      backgroundColor: Colors.red,
      behavior: SnackBarBehavior.floating,
    ),
  );
}

  Future<void> _login(BuildContext context) async {
    final response = await http.post(
      Uri.parse("$_baseUrl/login"),
      body: jsonEncode({
        "email": _emailController.text,
        "password": _passwordController.text,
      }),
      headers: {"Content-Type": "application/json"},
    );

    if (response.statusCode > 399) {
      _showSnackbar("Login failed. Please check your credentials.");
    } else {
      final data = jsonDecode(response.body);
      await _storage.write(key: "token", value: data["token"]);
      final account = Account.fromJson(data["account"]);
      if (context.mounted) {
        Navigator.of(context).pushReplacement(
          MaterialPageRoute(builder: (context) => AccountScreen(account: account)),
        );
      }
    }
  }

  Future<void> _signUp() async {
    if (!isLongEnough || !hasUppercase || !hasNumber || !passwordsMatch || !isEmailValid) return;
    final url = Uri.parse("$_baseUrl/create");
    final response = await http.put(
      url,
      body: jsonEncode({
        "name": _usernameController.text,
        "email": _emailController.text,
        "password": _passwordController.text,
      }),
      headers: {"Content-Type": "application/json"},
    );

    if (response.statusCode > 399) {
      _showSnackbar("Failed to create account");
    } else {
      setState(() {
        isSignUp = false;
      });
    }
  }

 @override 
 Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.blueGrey[50],
      body: Center(
        child: Container(
          padding: const EdgeInsets.all(20),
          decoration: BoxDecoration(
            color: Colors.white,
            borderRadius: BorderRadius.circular(12),
            boxShadow: [
              BoxShadow(
                color: Colors.black26,
                blurRadius: 10,
                spreadRadius: 2,
              ),
            ],
          ),
          width: 300,
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                isSignUp ? 'Sign Up' : 'Login',
                style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
              ),
              const SizedBox(height: 20),
              if (isSignUp) ...[
                TextField(
                  controller: _usernameController,
                  decoration: InputDecoration(
                    labelText: 'Username',
                    border: OutlineInputBorder(),
                  ),
                ),
                const SizedBox(height: 10),
              ],
              TextField(
                controller: _emailController,
                onChanged: _validateEmail,
                decoration: InputDecoration(
                  labelText: 'Email',
                  border: OutlineInputBorder(
                    borderSide: BorderSide(color: isEmailValid || _emailController.text.isEmpty ? Colors.grey : Colors.red),
                  ),
                  errorText: _emailController.text.isNotEmpty && !isEmailValid ? 'Invalid email address' : null,
                ),
              ),
              const SizedBox(height: 10),
              TextField(
                controller: _passwordController,
                obscureText: true,
                onChanged: _validatePassword,
                decoration: InputDecoration(
                  labelText: 'Password',
                  border: OutlineInputBorder(),
                ),
              ),
              if (isSignUp) ...[
                const SizedBox(height: 10),
                TextField(
                  controller: _confirmPasswordController,
                  obscureText: true,
                  onChanged: _validateConfirmPassword,
                  decoration: InputDecoration(
                    labelText: 'Confirm Password',
                    border: OutlineInputBorder(),
                  ),
                ),
              ],
              if (_passwordController.text.isNotEmpty && isSignUp) ...[
                  Padding(
                    padding: const EdgeInsets.only(top: 8.0),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text("Password must contain:", style: TextStyle(fontWeight: FontWeight.bold)),
                        ...["At least 8 characters", "An uppercase letter", "A number", "Passwords must match"]
                            .asMap()
                            .entries
                            .map((entry) {
                          List<bool> conditions = [isLongEnough, hasUppercase, hasNumber, passwordsMatch];
                          return Row(
                            children: [
                              Icon(
                                conditions[entry.key] ? Icons.check : Icons.close,
                                color: conditions[entry.key] ? Colors.green : Colors.red,
                              ),
                              SizedBox(width: 5),
                              Text("• ${entry.value}"),
                            ],
                          );
                        }),
                      ],
                    ),
                  ),
                ],               
              const SizedBox(height: 20),
              ElevatedButton(
                onPressed: isFormValid ? () => (isSignUp ? _signUp() : _login(context)) : null,
                child: Text(isSignUp ? 'Sign Up' : 'Login'),
              ),
              TextButton(
                onPressed: toggleAuthMode,
                child: Text(
                    isSignUp ? 'Already have an account? Login' : 'No account? Sign Up'),
              ),
            ],
          ),
        ),
      ),
    );
  }}

