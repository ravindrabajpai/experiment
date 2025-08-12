import 'dart:convert';
import 'dart:html';

void main() {
  final content = querySelector('#content')!;
  final form = FormElement();

  final userInput = InputElement()..placeholder = 'Username';
  final passInput = InputElement()
    ..type = 'password'
    ..placeholder = 'Password';
  final submit = ButtonElement()..text = 'Login';

  form.children.addAll([userInput, passInput, submit]);
  content.children.add(form);

  submit.onClick.listen((_) async {
    final username = userInput.value ?? '';
    final password = passInput.value ?? '';

    try {
      final response = await HttpRequest.request(
        'http://localhost:3000/login',
        method: 'POST',
        requestHeaders: {'Content-Type': 'application/json'},
        sendData: jsonEncode({'username': username, 'password': password}),
      );

      if (response.status == 200) {
        content.text = 'This is an AI-created project.';
      } else {
        window.alert('Invalid credentials');
      }
    } catch (e) {
      window.alert('Error contacting server');
    }
  });
}
