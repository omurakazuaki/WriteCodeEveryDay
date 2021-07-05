import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { Todo, TodoService } from '../todo.service';

@Component({
  selector: 'app-todo-detail',
  templateUrl: './todo-detail.component.html',
  styleUrls: ['./todo-detail.component.scss']
})
export class TodoDetailComponent implements OnInit {
  todo: Todo|undefined;

  constructor(
    private route: ActivatedRoute,
    private todoService: TodoService) { }

  ngOnInit(): void {
    const routeParams = this.route.snapshot.paramMap;
    const id =  Number(routeParams.get('id'));
    console.log(id)
    this.todo = this.todoService.getTodo(id);
  }

}
