import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';

void main() => runApp(const TodoApp());

class TodoApp extends StatefulWidget {
  const TodoApp({Key? key}) : super(key: key);

  @override
  State<TodoApp> createState() => TodoState();
}

class Todo {
  int id;
  String title;
  DateTime deadline;
  String description;
  Todo({
    required this.id,
    required this.title,
    required this.deadline,
    required this.description,
  });
}

class TodoState extends State<TodoApp> {
  final List<Todo> _list = [
    Todo(
        id: 0,
        title: "test",
        deadline: DateTime.now(),
        description: "test todo description")
  ];
  int _selected = -1;
  int _sequence = 0;

  void save(Todo todo) {
    debugPrint(todo.toString());
    if (todo.id < 0) {
      _sequence++;
      todo.id = _sequence;
      _list.add(todo);
    } else {
      _list[todo.id] = todo;
    }
  }

  Widget todoList(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Todo List'),
      ),
      body: ListView.builder(
        padding: const EdgeInsets.all(16.0),
        itemBuilder: (context, i) {
          int index = i ~/ 2;
          if (i.isOdd) return const Divider();
          final todo = _list[index];
          return ListTile(
            title: Text(todo.title.toString(),
                style: Theme.of(context).textTheme.headline6),
            subtitle: Text(todo.description.toString()),
            onTap: () {
              _selected = index;
              Navigator.pushNamed(context, '/detail');
            },
          );
        },
        itemCount: _list.length * 2,
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          _selected = -1;
          Navigator.of(context).pushNamed('/edit');
        },
        tooltip: 'New Todo',
        child: const Icon(Icons.add),
      ),
    );
  }

  Widget todoDetail(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Todo Detail'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.start,
          children: [
            Text(
              _list[_selected].title.toString(),
              style: Theme.of(context).textTheme.headline4,
            ),
            Text(
              _list[_selected].deadline.toIso8601String(),
            ),
            Text(
              _list[_selected].description.toString(),
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => Navigator.of(context).pushNamed('/edit'),
        tooltip: 'Edit Todo',
        child: const Icon(Icons.edit),
      ),
    );
  }

  Widget todoEdit(BuildContext context) {
    String _title = _selected < 0 ? '' : _list[_selected].title;
    DateTime _deadline =
        _selected < 0 ? DateTime.now() : _list[_selected].deadline;
    String _description = _selected < 0 ? '' : _list[_selected].title;
    final _formKey = GlobalKey<FormState>();
    final _deadlineKey = GlobalKey<FormState>();
    return Scaffold(
      appBar: AppBar(
        title: const Text('Todo Edit'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.start,
          children: [
            Form(
              key: _formKey,
              child: Column(
                mainAxisAlignment: MainAxisAlignment.start,
                children: [
                  TextFormField(
                    decoration: const InputDecoration(
                      labelText: "Title",
                    ),
                    initialValue: _title,
                    onChanged: (v) => _title = v,
                    validator: (v) {
                      if (v == null || v.isEmpty) {
                        return "Title is required";
                      }
                      return null;
                    },
                  ),
                  TextFormField(
                    decoration: const InputDecoration(
                      labelText: "Descripton",
                    ),
                    initialValue: _description,
                    onChanged: (v) => _description = v,
                    validator: (v) {
                      if (v == null || v.isEmpty) {
                        return "Descripton is required";
                      }
                      return null;
                    },
                  ),
                  TextFormField(
                    key: _deadlineKey,
                    decoration: const InputDecoration(
                      labelText: "DeadLine",
                    ),
                    initialValue: _deadline.toIso8601String(),
                    onTap: () {
                      pickDate(context).then((value) {
                        if (value != null) {
                          _deadline = value;
                        }
                      });
                    },
                  ),
                ],
              ),
            ),
            const Padding(
              padding: EdgeInsets.all(16),
            ),
            ElevatedButton(
              onPressed: () {
                if (_formKey.currentState!.validate()) {
                  save(Todo(
                    id: _selected,
                    title: _title,
                    deadline: _deadline,
                    description: _description,
                  ));
                  Navigator.of(context).popUntil((route) => route.isFirst);
                  Navigator.of(context).pushReplacementNamed("/list");
                }
              },
              child: const Text("SAVE"),
            )
          ],
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Todo App',
      theme: ThemeData(
        primarySwatch: Colors.blueGrey,
      ),
      initialRoute: '/list',
      routes: <String, WidgetBuilder>{
        '/list': todoList,
        '/detail': todoDetail,
        '/edit': todoEdit,
      },
      localizationsDelegates: const [
        GlobalMaterialLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
      ],
      supportedLocales: const [
        Locale('en'),
        Locale('ja'),
      ],
    );
  }
}

Future<DateTime?> pickDate(BuildContext context) async {
  final DateTime? picked = await showDatePicker(
    context: context,
    initialDate: DateTime.now(),
    firstDate: DateTime(DateTime.now().year),
    lastDate: DateTime(DateTime.now().year + 6),
    locale: const Locale('ja'),
  );
  return picked;
}
