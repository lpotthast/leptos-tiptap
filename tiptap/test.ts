/*
 * This file is irrelevant for building tiptap and can be ignored.
 * It simply exists to check out the JS Tiptap API through its typescript type definitions.
 */

import { Editor } from "@tiptap/core";
import StarterKit from "@tiptap/starter-kit";
import Strike, { StrikeOptions } from '@tiptap/extension-strike'

var editor: Editor = new Editor({
    element: document.getElementById('asd') || undefined,
    extensions: [
      StarterKit,
    ],
    content: '<p>Hello World!</p>',
  });

editor.setEditable(true);

editor.chain().toggleBlockquote();
editor.chain().toggleBold().run();
editor.chain().toggleBulletList();
editor.chain().toggleCode();
editor.chain().toggleCodeBlock();
editor.chain().toggleHeading({ level: 1 });
editor.chain().toggleItalic();
editor.chain().toggleOrderedList();
editor.chain().toggleStrike();
//editor.chain().toggleWrap();
//editor.chain().toggleList();
//editor.chain().toggleMark();
//editor.chain().toggleNode();

editor.chain().clearContent();
editor.chain().clearNodes();
editor.chain().redo();
editor.chain().scrollIntoView();
editor.chain().setHardBreak();
editor.chain().setHorizontalRule();
editor.chain().setParagraph();
editor.chain().undo();

editor.destroy()
