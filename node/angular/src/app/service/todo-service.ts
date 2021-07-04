type Todo = {
  id: number,
  title: string,
  description: string
}

const todoList: Todo[] = [
    {id: 1, title: 'test', description: 'This is a test todo.'},
    {id: 2, title: 'Tel', description: 'This is a test todo.'},
];

const getTodoList = () :Todo[] => {
  return todoList;
}

const getTodo = (id: number) : Todo | undefined => {
  return todoList.find(t => t.id === id);
}

export {
  Todo,
  getTodoList,
  getTodo
}
