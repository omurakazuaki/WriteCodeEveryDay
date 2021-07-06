import { Injectable } from '@angular/core';

export type Todo = {
  id: number,
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
    return this.todoList.find(t => t.id === id);
  }
  setDone(id: number, done: boolean) {
    const todo = this.getTodo(id);
    if (todo !== undefined) {
      todo.done = done;
    }
  }

}
