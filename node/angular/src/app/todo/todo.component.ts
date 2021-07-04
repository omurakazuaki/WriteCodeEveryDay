import { Component, OnInit } from '@angular/core';
import { Todo, getTodoList } from '../service/todo-service';

@Component({
  selector: 'app-todo',
  templateUrl: './todo.component.html',
  styleUrls: ['./todo.component.scss']
})
export class TodoComponent implements OnInit {
  todoList: Todo[]|undefined;

  constructor() { }

  ngOnInit(): void {
    this.todoList = getTodoList();
  }

  onTest() {
    alert('test');
  }

}
