import { createSlice, PayloadAction } from '@reduxjs/toolkit';

export interface Todo {id: number | null, title: string, deadline: string};

export interface TodoState {
  list: Todo[];
  selected: Todo | null;
  sequence: number;
}

const initialState: TodoState = {
  list: [
    {id: 1, title : 'Tel', deadline: 'Jan 9, 2014'},
    {id: 2, title : 'Programming', deadline: 'Jan 7, 2014'},
    {id: 3, title : 'Meeting', deadline: 'July 20, 2014'},
  ],
  selected: null,
  sequence: 3
};

export const todoSlice = createSlice({
  name: 'todo',
  initialState,
  reducers: {
    update: (state, action: PayloadAction<Todo | null>) => {
      if (!action.payload) return;
      const target = state.list.find(t => t.id === action.payload?.id);
      if (target) {
        target.title = action.payload?.title;
        target.deadline = action.payload?.deadline;
      } else {
        state.sequence += 1;
        action.payload.id = state.sequence;
        state.list.push(action.payload);
      }
    },
    select: (state, action: PayloadAction<Number | null>) => {
      state.selected = state.list.find(t => t.id === action.payload) || {id: null, title: '', deadline: ''};
    }
  },
});

export const { update, select } = todoSlice.actions;

export default todoSlice.reducer;
