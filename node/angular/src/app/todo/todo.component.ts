import { Component, OnInit } from '@angular/core';
import { Todo, TodoService } from '../todo.service';

@Component({
  selector: 'app-todo',
  templateUrl: './todo.component.html',
  styleUrls: ['./todo.component.scss']
})
export class TodoComponent implements OnInit {
  todoList: Todo[]|undefined;

  constructor(
    private todoService: TodoService) { }

  ngOnInit(): void {
    this.todoList = this.todoService.getTodoList();
  }

  onTest() {
    alert('test');
  }

}
