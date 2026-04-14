import { ChangeEvent, JSX, useState } from 'react';
import { Button, Form } from 'react-bootstrap';
import { toast } from 'react-toastify';

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
    <>
      {fileSelectedMessage(file!)}
      <Button
        variant={'outline-info'}
        size={'sm'}
        onClick={() => setFileAccepted(false)}
      >
        Change file
      </Button>
    </>
  ) : (
    <Form.File
      disabled={disabled}
      label={label}
      custom
      accept={accept}
      onChange={async (event: ChangeEvent<HTMLInputElement>) => {
        try {
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
        } catch (e) {
          console.error(
            `Failed to handle file ${event?.target?.files?.[0].name}: ${e}`,
          );
          if (`${e}`.toLowerCase().includes('ole signature'))
            toast.error(
              "File is invalid or corrupt. Please try using 'Save as...' to create a new copy and try again.",
            );
        }
      }}
    />
  );
}
