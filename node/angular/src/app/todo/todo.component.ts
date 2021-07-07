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

  onChangeDone(event: any, todo: Todo) {
    console.log(event.target.checked)
    if (todo.id) {
      this.todoService.setDone(todo.id, event.target.checked);
      this.todoList = this.todoService.getTodoList();
    }
  }

}
