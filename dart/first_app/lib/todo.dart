import 'package:flutter/material.dart';

void main() => runApp(const TodoApp());

class TodoApp extends StatefulWidget {
  const TodoApp({Key? key}) : super(key: key);

  @override
  State<TodoApp> createState() => TodoState();
}

class Todo {
  int? id;
  String? title;
  String? descripton;
  Todo({this.id, this.title, this.descripton});
}

class TodoState extends State<TodoApp> {
  final List<Todo> _list = [
    Todo(id: 0, title: "test", descripton: "test todo description")
  ];
  int _selected = 0;
  int _sequence = 0;

  void add(Todo todo) {
    _sequence++;
    todo.id = _sequence;
    _list.add(todo);
  }

  Widget todoList(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Todo List'),
      ),
      body: ListView.builder(
        padding: const EdgeInsets.all(16.0),
        itemBuilder: (context, i) {
          if (i.isOdd) return const Divider();
          final todo = _list[i];
          return ListTile(
            title: Text(todo.title.toString(),
                style: Theme.of(context).textTheme.headline6),
            subtitle: Text(todo.descripton.toString()),
            onTap: () {
              _selected = i;
              Navigator.pushNamed(context, '/detail');
            },
          );
        },
        itemCount: _list.length * 2,
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () => Navigator.of(context).pushNamed('/detail'),
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
              _list[_selected].descripton.toString(),
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
    return Scaffold(
      appBar: AppBar(
        title: const Text('Todo Edit'),
      ),
      body: Container(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.start,
          children: [
            TextFormField(
              decoration: const InputDecoration(
                labelText: "Title",
              ),
              initialValue: _list[_selected].title.toString(),
              onChanged: (v) => _list[_selected].title = v,
            ),
            TextFormField(
              decoration: const InputDecoration(
                labelText: "Descripton",
              ),
              initialValue: _list[_selected].descripton.toString(),
              onChanged: (v) => _list[_selected].descripton = v,
            ),
            TextButton(
              onPressed: () {
                Navigator.of(context).pushReplacementNamed("/detail");
              },
              child: const Text("SAVE"),
            ),
          ],
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Todo App',
      initialRoute: '/list',
      routes: <String, WidgetBuilder>{
        '/list': todoList,
        '/detail': todoDetail,
        '/edit': todoEdit,
      },
    );
  }
}
