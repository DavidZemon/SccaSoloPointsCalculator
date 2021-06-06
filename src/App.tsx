import { ChangeEvent, Component, ComponentPropsWithoutRef } from 'react';
import 'bootswatch/dist/darkly/bootstrap.css';
import { Button, Col, Container, Form, Row } from 'react-bootstrap';
import { ClassResultsProcessor } from './services';

interface AppState {
  eventResultsFile?: File;
}

class App extends Component<ComponentPropsWithoutRef<any>, AppState> {
  private readonly processor = new ClassResultsProcessor();
  constructor(props: Readonly<React.ComponentPropsWithoutRef<any>>) {
    super(props);
    this.state = {};
  }

  render() {
    return (
      <Container>
        <Row>
          <Col>
            <h1>SCCA Solo Points Calculator</h1>
          </Col>
        </Row>
        <Row>
          <Col>
            {this.state.eventResultsFile ? (
              <p>
                Ready to process <code>{this.state.eventResultsFile.name}</code>
              </p>
            ) : (
              <Form.File
                label="Event Class Results"
                custom
                onChange={(event: ChangeEvent<HTMLInputElement>) => {
                  if (event.target.files && event.target.files.length) {
                    this.setState({
                      eventResultsFile: event.target.files[0],
                    });
                  }
                }}
              />
            )}
          </Col>
        </Row>

        <Row>
          <Col>
            <Button
              disabled={this.state.eventResultsFile === undefined}
              variant={'primary'}
              onClick={async () => {
                await this.processor.process(
                  await this.state.eventResultsFile!.text(),
                );
              }}
            >
              Process
            </Button>
          </Col>
        </Row>
      </Container>
    );
  }
}

export default App;
