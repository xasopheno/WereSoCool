import { createContext } from 'react';
import { Fetch } from '../store';
import axios from 'axios';
import FileSaver from 'file-saver';
import { settings } from '../settings';
import AceEditor from 'react-ace';
import { remote } from 'electron';
import path from 'path';

export enum ResponseType {
  RenderSuccess = 'RenderSuccess',
  IdError = 'IdError',
  ParseError = 'ParseError',
  IndexError = 'IndexError',
  MsgError = 'Msg',
}

export type Action =
  | { _k: 'Set_Editor_Focus' }
  | { _k: 'Increment_Editor_Type'; editor: number }
  | { _k: 'Set_Editor_Ref'; editor_ref: AceEditor }
  | { _k: 'Backend'; fetch: Fetch }
  | { _k: 'Set_Render_State'; state: ResponseType }
  | { _k: 'Set_Markers'; line: number; column: number; n_lines: number }
  | { _k: 'Reset_Markers' }
  | { _k: 'Set_Error_Message'; message: string }
  | { _k: 'Reset_Error_Message' }
  | { _k: 'Set_Language'; language: string }
  | { _k: 'Reset_Language' };

export class Dispatch {
  constructor(public dispatch: React.Dispatch<Action>) {}

  setEditorFocus = (editor_ref: AceEditor | null): void => {
    if (editor_ref) {
      editor_ref.editor.focus();
    }
  };

  onSetEditorRef(editor_ref: AceEditor): void {
    this.dispatch({
      _k: 'Set_Editor_Ref',
      editor_ref,
    });
  }

  async onDemo(filename: string, folder: string): Promise<void> {
    const fs = remote.require('fs');

    const demoPath = remote.app.isPackaged
      ? path.join(process.resourcesPath, `extraResources/${folder}`)
      : `./extraResources/${folder}`;

    try {
      const data = fs.readFileSync(`${demoPath}/${filename}`, 'utf-8');
      this.dispatch({ _k: 'Set_Language', language: data });
      await this.onRender(data);
    } catch (e) {
      console.log(e);
      this.dispatch({ _k: 'Backend', fetch: { state: 'bad', error: e } });
    }
  }

  onFileLoad = (e: React.ChangeEvent<HTMLInputElement>): void => {
    if (e && e.target && e.target.files && e.target.files.length > 0) {
      const file = e.target.files[0];
      const reader = new FileReader();
      reader.onload = (read_event: ProgressEvent<FileReader>) => {
        if (
          read_event.target &&
          read_event.target.result &&
          typeof read_event.target.result === 'string'
        ) {
          this.onUpdateLanguage(read_event.target.result);
        }
      };

      reader.readAsText(file);
    }
  };

  onFileSave(language: string): void {
    const blob = new Blob([language], {
      // type: '.socool',
      type: 'text/plain;charset=utf-8',
    });
    FileSaver.saveAs(blob, 'my_song.socool');
  }

  onIncrementEditorType(current_editor: number): void {
    const editor = (current_editor + 1) % 3;
    localStorage.setItem('editor', editor.toString());
    this.dispatch({
      _k: 'Increment_Editor_Type',
      editor,
    });
  }

  onUpdateLanguage(language: string): void {
    this.dispatch({ _k: 'Set_Language', language });
    localStorage.setItem('language', language);
  }

  onStop = async (): Promise<void> => {
    const stop_lang = `{ f: 220, l: 1, g: 1, p: 0 }\nmain = {Fm 0}`;
    await this.onRender(stop_lang);
  };

  onResetLanguage = (): void => {
    this.dispatch({ _k: 'Reset_Language' });
  };

  async onRender(language: string): Promise<void> {
    this.dispatch({ _k: 'Backend', fetch: { state: 'loading' } });

    try {
      const response = await axios.post(settings.backendURL, {
        language,
      });

      this.dispatch({ _k: 'Backend', fetch: { state: 'good' } });
      generateDispatches(response.data, language).map((dispatch) => {
        this.dispatch(dispatch);
      });
    } catch (e) {
      this.dispatch({ _k: 'Backend', fetch: { state: 'bad', error: e } });
    }
  }
}

const generateDispatches = (
  response: ResponseType,
  language: string
): Action[] => {
  const responseType = Object.keys(response)[0];
  // This should eventually be typed.
  // eslint-disable-next-line
  const value: any = Object.values(response)[0];
  const result: Action[] = [];
  // console.log(responseType);
  // console.log(value);
  switch (responseType) {
    case ResponseType.RenderSuccess:
      result.push({
        _k: 'Set_Render_State',
        state: ResponseType.RenderSuccess,
      });
      result.push({ _k: 'Reset_Error_Message' });
      result.push({ _k: 'Reset_Markers' });
      break;
    case ResponseType.ParseError:
      result.push({
        _k: 'Set_Render_State',
        state: ResponseType.ParseError,
      });
      result.push({
        _k: 'Set_Markers',
        line: value.line,
        column: value.column,
        n_lines: language.split('\n').length,
      });
      result.push({
        _k: 'Set_Error_Message',
        message: `Line: ${value.line} | Column ${value.column}`,
      });
      break;
    case ResponseType.IdError:
      result.push({
        _k: 'Set_Render_State',
        state: ResponseType.IdError,
      });
      result.push({
        _k: 'Set_Error_Message',
        message: `${value.id}`,
      });
      break;
    case ResponseType.IndexError:
      result.push({
        _k: 'Set_Render_State',
        state: ResponseType.IndexError,
      });
      result.push({
        _k: 'Set_Error_Message',
        message: `${value.message}`,
      });
      break;
    case ResponseType.MsgError:
      console.log(value);
      result.push({
        _k: 'Set_Render_State',
        state: ResponseType.MsgError,
      });
      result.push({
        _k: 'Set_Error_Message',
        message: `${value.message}`,
      });
      break;
    default:
      console.log('Unhandled error');
      console.log(response);
      break;
  }
  return result;
};

export const DispatchContext = createContext(
  (undefined as unknown) as Dispatch
);
