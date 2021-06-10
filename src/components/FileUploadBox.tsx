import { ChangeEvent, Component, ComponentPropsWithoutRef } from 'react';
import { Form } from 'react-bootstrap';

interface FileUploadBoxProps extends ComponentPropsWithoutRef<any> {
  label: string;
  file?: File;
  onFileSelect: ((f: File) => boolean) | ((f: File) => Promise<boolean>);
  fileSelectedMessage: (f: File) => JSX.Element | JSX.Element[];
}

interface FileUploadState {
  fileAccepted: boolean;
}

export class FileUploadBox extends Component<
  FileUploadBoxProps,
  FileUploadState
> {
  constructor(props: Readonly<FileUploadBoxProps> | FileUploadBoxProps) {
    super(props);
    this.state = { fileAccepted: false };
  }

  render() {
    return this.state.fileAccepted ? (
      this.props.fileSelectedMessage(this.props.file!)
    ) : (
      <Form.File
        label={this.props.label}
        custom
        onChange={async (event: ChangeEvent<HTMLInputElement>) => {
          if (
            event.target.files &&
            event.target.files.length &&
            event.target.files[0]
          ) {
            let accepted = this.props.onFileSelect(event.target.files[0]);
            // @ts-expect-error
            if (accepted.then) {
              accepted = await accepted;
            }
            this.setState({
              // @ts-expect-error
              fileAccepted: accepted,
            });
          }
        }}
      />
    );
  }
}
