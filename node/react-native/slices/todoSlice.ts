import { Platform } from 'react-native';
import { createAsyncThunk, createSlice, PayloadAction } from '@reduxjs/toolkit';
import Storage from 'react-native-storage';
import AsyncStorage from '@react-native-community/async-storage';

export interface Todo {id: number | null,　title: string, subtitle: string};

export interface TodoState {
  list: Todo[];
  selected: Todo;
}

const storage = new Storage({
  storageBackend: Platform.OS == 'web' ? window.localStorage : AsyncStorage,
  enableCache: true
});

const initialState: TodoState = {
  list: [],
  selected: {id: null, title: '', subtitle: ''},
};

const key = 'WriteCodeEveryDayReactNativeTodoList';

export const fetchAllTodo = createAsyncThunk(
  "todo/loadAllTodo",
  async () => {
    return await storage.load({key}) as Todo[];
  }
);

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
      storage.save({key, data: state.list});
    },
    updateSelected: (state, action: PayloadAction<Todo>) => {
      state.selected = action.payload;
    },
    select: (state, action: PayloadAction<Number | null>) => {
      const find = state.list.find(t => t.id === action.payload);
      state.selected = find ? {...find} : {id: null, title: '', subtitle: ''};
    }
  },
  extraReducers: (builder) => {
    builder.addCase(fetchAllTodo.fulfilled, (state, action) => {
      state.list = action.payload;
    });
  }
});

export const { save, select, updateSelected } = todoSlice.actions;

export default todoSlice.reducer;
