import { ChangeEvent, JSX, useState } from 'react';
import { Form } from 'react-bootstrap';

interface FileUploadBoxProps {
  disabled?: boolean;
  label: string;
  file?: File;
  onFileSelect: ((f: File) => boolean) | ((f: File) => Promise<boolean>);
  fileSelectedMessage: (f: File) => JSX.Element;
  accept?: string;
}

export function FileUploadBox({
  disabled,
  label,
  file,
  onFileSelect,
  fileSelectedMessage,
  accept,
}: FileUploadBoxProps): JSX.Element {
  const [fileAccepted, setFileAccepted] = useState(false);

  return fileAccepted ? (
    fileSelectedMessage(file!)
  ) : (
    <Form.File
      disabled={disabled}
      label={label}
      custom
      accept={accept}
      onChange={async (event: ChangeEvent<HTMLInputElement>) => {
        if (
          event.target.files &&
          event.target.files.length &&
          event.target.files[0]
        ) {
          let accepted = onFileSelect(event.target.files[0]);
          // @ts-expect-error
          if (accepted.then) {
            accepted = await accepted;
          }
          // @ts-expect-error
          setFileAccepted(accepted);
        }
      }}
    />
  );
}
