import { HttpContextContract } from '@ioc:Adonis/Core/HttpContext'

type Todo = {
  id?: number,
  title: string,
  description: string,
  done: boolean
}
const todoList: Todo[] = [
  {id: 1, title: 'test', description: 'This is a test todo.', done: true},
  {id: 2, title: 'Tel', description: 'This is a test todo.', done: false},
];

export default class TodoController {

  public async getTodoList(ctx: HttpContextContract) {
    return todoList
  }

  public async getTodo(ctx: HttpContextContract) {
    return Object.assign({}, todoList.find(t => t.id === Number(ctx.params.id)));
  }

  public async save(ctx: HttpContextContract) {
    const todo = ctx.request.body() as Todo;
    if (todo.id === undefined) {
      todo.id = todoList.reduce((acc, t) => {
        return t.id && acc < t.id ? t.id : acc;
      }, 0) + 1;
      todoList.push(todo);
    } else {
      const before = todoList.find(t => t.id === todo.id);
      Object.assign(before, todo);
    }
    return todo;
  }

}
