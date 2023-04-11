import { writable } from "svelte/store";

export interface Project {
  root: string;
}

export const project = writable<Project | null>(null);

export interface Shell {
  selectedFile: string | undefined;
}

const createShell = () => {
  const { subscribe, set, update } = writable<Shell>({
    selectedFile: undefined,
  });

  return {
    subscribe,
    selectFile(path: string | undefined) {
      update((shell) => ({
        ...shell,
        selectedFile: path,
      }));
    },
  };
};

export const shell = createShell();
