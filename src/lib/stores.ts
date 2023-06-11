import { writable } from "svelte/store";

export interface Project {
  root: string;
}

export const project = writable<Project | null>(null);

export interface Shell {
  selectedFile: string | undefined;
  modals: Modal[];
}

export interface BaseModal {
  title: string;
}

export interface InputModal extends BaseModal {
  type: "input";
  placeholder?: string;
  initialText?: string;
  callback: (content: string | null) => void;
}

export interface ConfirmModal extends BaseModal {
  type: "confirm",
  callback: (canceled: boolean) => void
}

export type Modal = InputModal | ConfirmModal;

const createShell = () => {
  const { subscribe, set, update } = writable<Shell>({
    selectedFile: undefined,
    modals: [],
  });

  return {
    subscribe,
    selectFile(path: string | undefined) {
      update((shell) => ({
        ...shell,
        selectedFile: path,
      }));
    },
    createModal(modal: Modal) {
      update((shell) => ({
        ...shell,
        modals: [...shell.modals, modal],
      }));
    },
    popModal() {
      update((shell) => {
        const modals = shell.modals;
        modals.shift();
        return {
          ...shell,
          modals,
        };
      });
    },
  };
};

export const shell = createShell();
