import { createSlice, PayloadAction } from '@reduxjs/toolkit';

export interface Todo {id: number | null, title: string, deadline: string};

export interface TodoState {
  list: Todo[];
  selected: Todo | null;
  sequence: number;
}

const initialState: TodoState = {
  list: [
    {id: 1, title : 'Tel', deadline: '2021-03-14'},
    {id: 2, title : 'Programming', deadline: '2021-03-15'},
    {id: 3, title : 'Meeting', deadline: '2021-03-20'},
  ],
  selected: null,
  sequence: 3
};

export const todoSlice = createSlice({
  name: 'todo',
  initialState,
  reducers: {
    save: (state, action: PayloadAction<Todo>) => {
      if (!action.payload) return;
      const find = state.list.find(t => t.id === action.payload?.id);
      if (find) {
        Object.assign(find, action.payload);
      } else {
        state.sequence += 1;
        action.payload.id = state.sequence;
        state.list.push(action.payload);
        state.selected = action.payload;
      }
    },
    updateSelected: (state, action: PayloadAction<Todo>) => {
      state.selected = action.payload;
    },
    select: (state, action: PayloadAction<Number | null>) => {
      const find = state.list.find(t => t.id === action.payload);
      state.selected = find ? {...find} : {id: null, title: '', deadline: '2021-03-13'};
    }
  },
});

export const { save, select, updateSelected } = todoSlice.actions;

export default todoSlice.reducer;
