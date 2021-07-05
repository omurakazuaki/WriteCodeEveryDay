import { Injectable } from '@angular/core';

export type Todo = {
  id: number,
  title: string,
  description: string
}

@Injectable({
  providedIn: 'root'
})
export class TodoService {
  todoList: Todo[] = [
    {id: 1, title: 'test', description: 'This is a test todo.'},
    {id: 2, title: 'Tel', description: 'This is a test todo.'},
  ];
  constructor() { }
  public getTodoList() :Todo[] {
    return this.todoList;
  }
  public getTodo(id: number) : Todo | undefined {
    return this.todoList.find(t => t.id === id);
  }

}
