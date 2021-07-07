import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { Todo, TodoService } from '../todo.service';

@Component({
  selector: 'app-todo-detail',
  templateUrl: './todo-detail.component.html',
  styleUrls: ['./todo-detail.component.scss']
})
export class TodoDetailComponent implements OnInit {
  todo: Todo | undefined;
  edit: boolean = false;

  constructor(
    private route: ActivatedRoute,
    private todoService: TodoService) { }

  ngOnInit(): void {
    const routeParams = this.route.snapshot.paramMap;
    const queryParams = this.route.snapshot.queryParamMap;
    const id =  Number(routeParams.get('id'));
    this.edit = queryParams.has('edit');
    this.todo = id !== -1
      ? this.todoService.getTodo(id)
      : { title: '', description: '', done: false };
  }

  onEdit(): void {
    this.edit = true;
  }

  onSave(): void {
    if (this.todo) {
      this.todoService.save(this.todo);
    }
    this.edit = false;
  }

}
