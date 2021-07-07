import { Injectable } from '@angular/core';

export type Todo = {
  id?: number,
  title: string,
  description: string,
  done: boolean
}

@Injectable({
  providedIn: 'root'
})
export class TodoService {
  todoList: Todo[] = [
    {id: 1, title: 'test', description: 'This is a test todo.', done: true},
    {id: 2, title: 'Tel', description: 'This is a test todo.', done: false},
  ];
  constructor() { }
  getTodoList() :Todo[] {
    return this.todoList;
  }
  getTodo(id: number) : Todo | undefined {
    return Object.assign({}, this.todoList.find(t => t.id === id));
  }
  setDone(id: number, done: boolean) {
    const todo = this.getTodo(id);
    if (todo !== undefined) {
      todo.done = done;
    }
  }
  save(todo: Todo) {
    if (todo.id === undefined) {
      todo.id = this.todoList.reduce((acc, t) => {
        return t.id && acc < t.id ? t.id : acc;
      }, 0) + 1;
      this.todoList.push(todo);
    } else {
      const before = this.todoList.find(t => t.id === todo.id);
      Object.assign(before, todo);
    }
  }

}
