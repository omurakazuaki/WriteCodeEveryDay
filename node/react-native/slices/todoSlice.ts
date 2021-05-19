import { createSlice, PayloadAction } from '@reduxjs/toolkit';

export interface Todo {id: number | null,　title: string, subtitle: string};

export interface TodoState {
  list: Todo[];
  selected: Todo;
}

const initialState: TodoState = {
  list: [
    {
      id: 0,
      title: '新規TODO作成',
      subtitle: '空のTODO編集ページに遷移'
    },
    {
      id: 1,
      title: 'Redux',
      subtitle: 'reduxによる状態管理'
    },
    {
      id: 2,
      title: 'TODO編集',
      subtitle: '編集ページに遷移'
    },
    {
      id: 3,
      title: 'TODOデータを保存',
      subtitle: 'ローカルストレージにデータを保存する'
    },
    {
      id: 4,
      title: '認証',
      subtitle: 'ログイン時に認証する'
    },
    {
      id: 5,
      title: 'クローズ',
      subtitle: 'スワップしてクローズする'
    },
  ],
  selected: {id: null, title: '', subtitle: ''},
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
        action.payload.id = state.list.length;
        state.list.push(action.payload);
        state.selected = action.payload;
      }
    },
    updateSelected: (state, action: PayloadAction<Todo>) => {
      state.selected = action.payload;
    },
    select: (state, action: PayloadAction<Number | null>) => {
      const find = state.list.find(t => t.id === action.payload);
      state.selected = find ? {...find} : {id: null, title: '', subtitle: ''};
    }
  },
});

export const { save, select, updateSelected } = todoSlice.actions;

export default todoSlice.reducer;
