import { ChangeEvent, Component, ComponentPropsWithoutRef } from 'react';
import 'bootswatch/dist/darkly/bootstrap.css';
import { Button, Col, Container, Form, Row } from 'react-bootstrap';
import { ClassResultsProcessor } from './services';
import { EventResults } from './models';
import { EventResults as EventResultsComponent } from './components/EventResults';

interface AppState {
  eventResultsFile?: File;
  championshipResultsFile?: File;
  results?: EventResults;
}

class App extends Component<ComponentPropsWithoutRef<any>, AppState> {
  private readonly processor = new ClassResultsProcessor();
  constructor(props: Readonly<ComponentPropsWithoutRef<any>>) {
    super(props);
    this.state = {};
  }

  render() {
    return (
      <Container fluid>
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

          <Col>
            {this.state.championshipResultsFile ? (
              this.state.eventResultsFile ? (
                <p>
                  Ready to process{' '}
                  <code>{this.state.championshipResultsFile.name}</code>
                </p>
              ) : (
                <p>
                  <code>{this.state.championshipResultsFile.name}</code> set as
                  championship standings. Please add event results to begin
                  processing.
                </p>
              )
            ) : (
              <Form.File
                label="Championship standings"
                custom
                onChange={(event: ChangeEvent<HTMLInputElement>) => {
                  if (event.target.files && event.target.files.length) {
                    this.setState({
                      championshipResultsFile: event.target.files[0],
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
                const results = await this.processor.process(
                  await this.state.eventResultsFile!.text(),
                );
                this.setState({ results });
              }}
            >
              Process
            </Button>
          </Col>
        </Row>

        <EventResultsComponent results={this.state.results} />
      </Container>
    );
  }
}

export default App;
