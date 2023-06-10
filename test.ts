import { Editor } from "@tiptap/core";
import StarterKit from "@tiptap/starter-kit";
import Strike, { StrikeOptions } from '@tiptap/extension-strike'

var editor: Editor = new Editor({
    element: document.querySelector('#asd'),
    extensions: [
      StarterKit,
    ],
    content: '<p>Hello World!</p>',
  });

editor.chain().toggleBlockquote();
editor.chain().toggleBold();
editor.chain().toggleBulletList();
editor.chain().toggleCode();
editor.chain().toggleCodeBlock();
editor.chain().toggleHeading();
editor.chain().toggleItalic();
editor.chain().toggleList();
editor.chain().toggleMark();
editor.chain().toggleNode();
editor.chain().toggleOrderedList();
editor.chain().toggleStrike();
editor.chain().toggleWrap();

editor.chain().clearContent();
editor.chain().clearNodes();
editor.chain().redo();
editor.chain().scrollIntoView();
editor.chain().setHardBreak();
editor.chain().setHorizontalRule();
editor.chain().setParagraph();
editor.chain().undo();