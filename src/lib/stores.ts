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
  callback: (content: string | null) => void;
}

export type Modal = InputModal;

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
