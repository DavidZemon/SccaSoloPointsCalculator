import { Component, ComponentPropsWithoutRef } from 'react';

interface RamDownloadProps extends ComponentPropsWithoutRef<any> {
  filename?: string;
  content?: BlobPart;
  contentType?: string;
  downloadComplete?: () => void;
}

interface RamDownloadState {
  downloadUrl?: string;
}

export class RamDownload extends Component<RamDownloadProps, RamDownloadState> {
  private element: HTMLAnchorElement | null = null;

  constructor(props: Readonly<RamDownloadProps> | RamDownloadProps) {
    super(props);
    this.state = {};
  }

  componentDidUpdate(
    prevProps: Readonly<RamDownloadProps>,
    prevState: Readonly<RamDownloadState>,
  ) {
    // Don't bother taking any action if we don't have all the info we need
    if (this.props.filename && this.props.content && this.props.contentType) {
      console.log('Triggering download');

      // If content or content type changed, update the download URL
      if (
        this.props.content !== prevProps.content ||
        this.props.contentType !== prevProps.contentType
      )
        this.setState({
          downloadUrl: URL.createObjectURL(
            new Blob([this.props.content], { type: this.props.contentType }),
          ),
        });

      // Once the download URL changes, perform the download by initiating a click
      if (this.state.downloadUrl !== prevState.downloadUrl) {
        this.element!.click();
        this.props.downloadComplete?.();
      }
    }
  }

  render() {
    return (
      <a
        style={{ display: 'none' }}
        download={this.props.filename}
        href={this.state.downloadUrl}
        ref={(e) => (this.element = e!)}
      >
        Invisible link to trigger download
      </a>
    );
  }
}
