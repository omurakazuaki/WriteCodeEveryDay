import { Component, OnInit } from '@angular/core';

@Component({
  selector: 'app-todo',
  templateUrl: './todo.component.html',
  styleUrls: ['./todo.component.scss']
})
export class TodoComponent implements OnInit {
  todoList = [
    {title: 'test', description: 'This is a test todo.'},
    {title: 'Tel', description: 'This is a test todo.'},
  ];

  constructor() { }

  ngOnInit(): void {
  }

  onTest() {
    alert('test');
  }

}
