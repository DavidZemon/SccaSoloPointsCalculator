import { EOL } from 'os';
import assert from 'assert';
import 'react-toastify/dist/ReactToastify.css';
import 'bootswatch/dist/slate/bootstrap.css';
import { Component, ComponentPropsWithoutRef } from 'react';
import { Button, Col, Container, Row, Spinner } from 'react-bootstrap';
import { toast, ToastContainer } from 'react-toastify';
import parse from 'csv-parse/lib/sync';
import {
  ChampionshipResultsParser,
  EventResultsParser,
  PaxService,
} from './services';
import { ChampionshipResults, ChampionshipType, EventResults } from './models';
import { EventResults as EventResultsComponent } from './components/EventResults';
import { FileUploadBox } from './components/FileUploadBox';
import { ChampionshipResults as ChampionshipResultsComponent } from './components/ChampionshipResults';

interface AppState {
  eventResultsFile?: File;
  championshipResultsFiles: Record<ChampionshipType, File | undefined>;

  processing: boolean;

  eventResults?: EventResults;
  championshipResults?: ChampionshipResults;
}

class App extends Component<ComponentPropsWithoutRef<any>, AppState> {
  private readonly paxService = new PaxService();
  private readonly eventResultsParser = new EventResultsParser();
  private readonly championshipResultsProcessor = new ChampionshipResultsParser(
    this.paxService,
  );
  constructor(props: Readonly<ComponentPropsWithoutRef<any>>) {
    super(props);
    this.state = {
      championshipResultsFiles: {
        Class: undefined,
        PAX: undefined,
        Novice: undefined,
        Ladies: undefined,
      },
      processing: false,
    };
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
          {/* Page header */}
          <Row>
            <Col>
              <h1>SCCA Solo Points Calculator</h1>
            </Col>
          </Row>

          {/* File upload boxes */}
          <Row>
            <Col>
              <FileUploadBox
                label={'Full Event Results (by class)'}
                file={this.state.eventResultsFile}
                onFileSelect={async (f) => {
                  try {
                    await this.validateUploadedEventResultsFile(f);
                    this.setState({ eventResultsFile: f });
                    return true;
                  } catch (e) {
                    console.error(e.message);
                    toast.error(
                      'File format does not match expected. Please export event results with raw times, grouped by class.',
                    );
                    return false;
                  }
                }}
                fileSelectedMessage={(f) => (
                  <p>
                    Ready to process <code>{f.name}</code> as new event results
                  </p>
                )}
              />
            </Col>

            <Col>
              {Object.keys(this.state.championshipResultsFiles).map(
                (championshipType, index) => (
                  <FileUploadBox
                    key={index}
                    label={`${championshipType} Championship Standings`}
                    file={
                      this.state.championshipResultsFiles[
                        championshipType as ChampionshipType
                      ]
                    }
                    onFileSelect={(f) => {
                      const newResults = {
                        ...this.state.championshipResultsFiles,
                      };
                      newResults[championshipType as ChampionshipType] = f;
                      this.setState({ championshipResultsFiles: newResults });
                      return true;
                    }}
                    fileSelectedMessage={(f) =>
                      this.state.eventResultsFile ? (
                        <p>
                          Ready to process <code>{f.name}</code> as{' '}
                          <strong>{championshipType}</strong> championship
                          standings
                        </p>
                      ) : (
                        <p>
                          <code>{f.name}</code> set as{' '}
                          <strong>{championshipType}</strong> championship
                          standings. Please add event results to begin
                          processing.
                        </p>
                      )
                    }
                  />
                ),
              )}
            </Col>
          </Row>

          {/* Process button */}
          <Row>
            <Col>
              <Button
                style={{ width: '150px' }}
                disabled={this.state.eventResultsFile === undefined}
                variant={'primary'}
                onClick={async () => {
                  this.setState({ processing: true });
                  const eventResults = await this.eventResultsParser.parse(
                    await this.state.eventResultsFile!.text(),
                  );
                  const championshipResults =
                    await this.championshipResultsProcessor.parse(
                      this.state.championshipResultsFiles,
                      eventResults,
                    );
                  this.setState({
                    eventResults,
                    championshipResults,
                    processing: false,
                  });
                }}
              >
                {this.state.processing ? (
                  <Spinner animation={'border'} />
                ) : (
                  <span>Process</span>
                )}
              </Button>
            </Col>
          </Row>

          <EventResultsComponent
            paxService={this.paxService}
            results={this.state.eventResults}
          />

          <ChampionshipResultsComponent
            paxService={this.paxService}
            results={this.state.championshipResults}
          />
        </Container>
      </div>
    );
  }

  async validateUploadedEventResultsFile(f: File) {
    const content = await f.text();
    const firstLine = content.split(EOL)[0];
    const rows: string[][] = parse(firstLine, {
      columns: false,
      ltrim: true,
      rtrim: true,
      relaxColumnCount: true,
    });
    const firstLineWords = rows[0];

    const errorMessage = `First line does not match expected value. Actual: \`${firstLine}\``;

    assert(firstLineWords[0] === 'Results', errorMessage);
    assert(firstLineWords[2] === 'www.ProntoTimingSystem.com', errorMessage);
    assert(firstLineWords[3] === 'Pos', errorMessage);
    assert(firstLineWords[4] === 'Nbr', errorMessage);
    assert(firstLineWords[5] === "Driver's name, Town", errorMessage);
    assert(firstLineWords[6] === 'Car, Sponsor', errorMessage);
  }
}

export default App;
