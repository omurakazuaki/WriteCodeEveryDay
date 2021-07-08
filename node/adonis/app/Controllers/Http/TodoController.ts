import { HttpContextContract } from '@ioc:Adonis/Core/HttpContext'

export default class TodoController {
  todoList = [
    {id: 1, title: 'test', description: 'This is a test todo.', done: true},
    {id: 2, title: 'Tel', description: 'This is a test todo.', done: false},
  ];
  public async getTodoList(ctx: HttpContextContract) {
    return this.todoList
  }

  public async getTodo(ctx: HttpContextContract) {
    return Object.assign({}, this.todoList.find(t => t.id === Number(ctx.params.id)));
  }
}
