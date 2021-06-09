import { ChangeEvent, Component, ComponentPropsWithoutRef } from 'react';
import 'react-toastify/dist/ReactToastify.css';
import 'bootswatch/dist/darkly/bootstrap.css';
import { Button, Col, Container, Form, Row } from 'react-bootstrap';
import { toast } from 'react-toastify';
import { ClassResultsProcessor } from './services';
import { EventResults } from './models';
import { EventResults as EventResultsComponent } from './components/EventResults';
import { PaxService } from './services/PaxService';
import { EOL } from 'os';
import assert from 'assert';
import { ToastContainer } from 'react-toastify';

interface AppState {
  eventResultsFile?: File;
  championshipResultsFile?: File;
  results?: EventResults;
}

class App extends Component<ComponentPropsWithoutRef<any>, AppState> {
  private readonly processor = new ClassResultsProcessor();
  private readonly paxService = new PaxService();
  constructor(props: Readonly<ComponentPropsWithoutRef<any>>) {
    super(props);
    this.state = {};
  }

  async componentDidMount() {
    try {
      await this.paxService.init();
    } catch (e) {
      console.error(e);
    }
  }

  render() {
    return (
      <div>
        <ToastContainer pauseOnHover />
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
                  Ready to process{' '}
                  <code>{this.state.eventResultsFile.name}</code>
                </p>
              ) : (
                <Form.File
                  label="Event Class Results"
                  custom
                  onChange={async (event: ChangeEvent<HTMLInputElement>) => {
                    if (event.target.files && event.target.files.length) {
                      try {
                        await this.validateUploadedFile(event.target.files[0]);
                        this.setState({
                          eventResultsFile: event.target.files[0],
                        });
                      } catch (e) {
                        console.error(e.message);
                        toast.error(
                          'File format does not match expected. Please export event results with raw times, grouped by class.',
                        );
                      }
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
                    <code>{this.state.championshipResultsFile.name}</code> set
                    as championship standings. Please add event results to begin
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

          <EventResultsComponent
            paxService={this.paxService}
            results={this.state.results}
          />
        </Container>
      </div>
    );
  }

  async validateUploadedFile(f: File) {
    const content = await f.text();
    const firstLine = content.split(EOL)[0];
    const firstLineWords = firstLine.split(',');

    const errorMessage = `First line does not match expected value. Actual: \`${firstLine}\``;

    assert(firstLineWords[0] === '"Results"', errorMessage);
    assert(firstLineWords[2] === '"www.ProntoTimingSystem.com"', errorMessage);
    assert(firstLineWords[3] === '"Pos"', errorMessage);
    assert(firstLineWords[4] === '"Nbr"', errorMessage);
    assert(firstLineWords[5] === '"Driver\'s name', errorMessage);
    assert(firstLineWords[6] === ' Town"', errorMessage);
    assert(firstLineWords[7] === '"Car', errorMessage);
    assert(firstLineWords[8] === ' Sponsor"', errorMessage);
  }
}

export default App;
